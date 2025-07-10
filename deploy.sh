#!/bin/bash

# Thai Energy Trading System - Kubernetes Deployment Script
# This script deploys the complete Thai Energy Trading System to Kubernetes

set -e

# Configuration
NAMESPACE="thai-energy-trading"
DOCKER_REGISTRY="your-registry.com"
IMAGE_TAG="latest"
KUBECTL_CONTEXT="your-k8s-context"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
BLUE='\033[0;34m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${BLUE}[INFO]${NC} $1"
}

print_success() {
    echo -e "${GREEN}[SUCCESS]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Function to check prerequisites
check_prerequisites() {
    print_status "Checking prerequisites..."
    
    # Check if kubectl is installed
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl is not installed. Please install kubectl first."
        exit 1
    fi
    
    # Check if docker is installed
    if ! command -v docker &> /dev/null; then
        print_error "docker is not installed. Please install docker first."
        exit 1
    fi
    
    # Check if we can connect to Kubernetes cluster
    if ! kubectl cluster-info &> /dev/null; then
        print_error "Cannot connect to Kubernetes cluster. Please check your kubeconfig."
        exit 1
    fi
    
    print_success "All prerequisites satisfied"
}

# Function to build and push Docker images
build_and_push_images() {
    print_status "Building Docker images..."
    
    # Build the main application image
    docker build -t ${DOCKER_REGISTRY}/thai-energy-trading:${IMAGE_TAG} .
    
    # Build the oracle service image (if exists)
    if [ -f "oracle/Dockerfile" ]; then
        docker build -t ${DOCKER_REGISTRY}/thai-energy-oracle:${IMAGE_TAG} ./oracle/
    fi
    
    print_status "Pushing images to registry..."
    docker push ${DOCKER_REGISTRY}/thai-energy-trading:${IMAGE_TAG}
    
    if [ -f "oracle/Dockerfile" ]; then
        docker push ${DOCKER_REGISTRY}/thai-energy-oracle:${IMAGE_TAG}
    fi
    
    print_success "Images built and pushed successfully"
}

# Function to create namespace if it doesn't exist
create_namespace() {
    print_status "Creating namespace if it doesn't exist..."
    
    if kubectl get namespace ${NAMESPACE} &> /dev/null; then
        print_warning "Namespace ${NAMESPACE} already exists"
    else
        kubectl create namespace ${NAMESPACE}
        print_success "Namespace ${NAMESPACE} created"
    fi
}

# Function to apply Kubernetes manifests
apply_manifests() {
    print_status "Applying Kubernetes manifests..."
    
    # Apply manifests in order
    kubectl apply -f k8s/01-namespace-configmap.yaml
    kubectl apply -f k8s/02-secrets-storage.yaml
    kubectl apply -f k8s/07-rbac-security.yaml
    kubectl apply -f k8s/03-validator-statefulset.yaml
    kubectl apply -f k8s/04-api-server-deployment.yaml
    kubectl apply -f k8s/05-oracle-deployment.yaml
    kubectl apply -f k8s/06-ingress-loadbalancer.yaml
    kubectl apply -f k8s/08-monitoring.yaml
    kubectl apply -f k8s/09-autoscaling-limits.yaml
    
    print_success "All manifests applied successfully"
}

# Function to wait for deployments to be ready
wait_for_deployments() {
    print_status "Waiting for deployments to be ready..."
    
    # Wait for StatefulSet
    kubectl rollout status statefulset/thai-energy-validator -n ${NAMESPACE} --timeout=600s
    
    # Wait for Deployments
    kubectl rollout status deployment/thai-energy-api-server -n ${NAMESPACE} --timeout=300s
    kubectl rollout status deployment/thai-energy-oracle -n ${NAMESPACE} --timeout=300s
    kubectl rollout status deployment/prometheus -n ${NAMESPACE} --timeout=300s
    kubectl rollout status deployment/grafana -n ${NAMESPACE} --timeout=300s
    
    print_success "All deployments are ready"
}

# Function to display deployment status
show_status() {
    print_status "Deployment Status:"
    echo ""
    
    echo "Pods:"
    kubectl get pods -n ${NAMESPACE}
    echo ""
    
    echo "Services:"
    kubectl get services -n ${NAMESPACE}
    echo ""
    
    echo "Ingress:"
    kubectl get ingress -n ${NAMESPACE}
    echo ""
    
    echo "PVCs:"
    kubectl get pvc -n ${NAMESPACE}
    echo ""
}

# Function to get access information
show_access_info() {
    print_status "Access Information:"
    echo ""
    
    # Get LoadBalancer IP
    LB_IP=$(kubectl get service thai-energy-loadbalancer -n ${NAMESPACE} -o jsonpath='{.status.loadBalancer.ingress[0].ip}' 2>/dev/null || echo "Pending")
    
    echo "🌐 External Access:"
    echo "   Load Balancer IP: ${LB_IP}"
    echo "   P2P Port: 30333"
    echo "   WebSocket Port: 9944"
    echo "   HTTP RPC Port: 8080"
    echo ""
    
    echo "📊 Monitoring:"
    echo "   Prometheus: kubectl port-forward svc/prometheus-service 9090:9090 -n ${NAMESPACE}"
    echo "   Grafana: kubectl port-forward svc/grafana-service 3000:3000 -n ${NAMESPACE}"
    echo "   Grafana Admin: admin / thai-energy-admin"
    echo ""
    
    echo "🔍 Useful Commands:"
    echo "   View logs: kubectl logs -f statefulset/thai-energy-validator -n ${NAMESPACE}"
    echo "   Check pods: kubectl get pods -n ${NAMESPACE}"
    echo "   Shell access: kubectl exec -it thai-energy-validator-0 -n ${NAMESPACE} -- /bin/bash"
    echo ""
}

# Function to run health checks
run_health_checks() {
    print_status "Running health checks..."
    
    # Check if all pods are running
    RUNNING_PODS=$(kubectl get pods -n ${NAMESPACE} --field-selector=status.phase=Running --no-headers | wc -l)
    TOTAL_PODS=$(kubectl get pods -n ${NAMESPACE} --no-headers | wc -l)
    
    if [ ${RUNNING_PODS} -eq ${TOTAL_PODS} ]; then
        print_success "All ${TOTAL_PODS} pods are running"
    else
        print_warning "${RUNNING_PODS}/${TOTAL_PODS} pods are running"
    fi
    
    # Check PVC status
    BOUND_PVCS=$(kubectl get pvc -n ${NAMESPACE} --no-headers | grep Bound | wc -l)
    TOTAL_PVCS=$(kubectl get pvc -n ${NAMESPACE} --no-headers | wc -l)
    
    if [ ${BOUND_PVCS} -eq ${TOTAL_PVCS} ]; then
        print_success "All ${TOTAL_PVCS} PVCs are bound"
    else
        print_warning "${BOUND_PVCS}/${TOTAL_PVCS} PVCs are bound"
    fi
}

# Function to cleanup deployment
cleanup() {
    print_status "Cleaning up deployment..."
    
    kubectl delete -f k8s/ --ignore-not-found=true
    kubectl delete namespace ${NAMESPACE} --ignore-not-found=true
    
    print_success "Cleanup completed"
}

# Main deployment function
deploy() {
    echo "🌟 Thai Energy Trading System - Kubernetes Deployment 🌟"
    echo "=========================================================="
    echo ""
    
    check_prerequisites
    
    if [ "${SKIP_BUILD}" != "true" ]; then
        build_and_push_images
    else
        print_warning "Skipping image build (SKIP_BUILD=true)"
    fi
    
    create_namespace
    apply_manifests
    wait_for_deployments
    run_health_checks
    show_status
    show_access_info
    
    print_success "🎉 Deployment completed successfully!"
}

# Parse command line arguments
case "${1}" in
    deploy)
        deploy
        ;;
    cleanup)
        cleanup
        ;;
    status)
        show_status
        ;;
    health)
        run_health_checks
        ;;
    access)
        show_access_info
        ;;
    *)
        echo "Usage: $0 {deploy|cleanup|status|health|access}"
        echo ""
        echo "Commands:"
        echo "  deploy   - Deploy the Thai Energy Trading System"
        echo "  cleanup  - Remove all deployed resources"
        echo "  status   - Show deployment status"
        echo "  health   - Run health checks"
        echo "  access   - Show access information"
        echo ""
        echo "Environment Variables:"
        echo "  DOCKER_REGISTRY - Docker registry URL (default: your-registry.com)"
        echo "  IMAGE_TAG       - Image tag (default: latest)"
        echo "  KUBECTL_CONTEXT - Kubernetes context (default: current context)"
        echo "  SKIP_BUILD      - Skip image build (default: false)"
        exit 1
        ;;
esac
