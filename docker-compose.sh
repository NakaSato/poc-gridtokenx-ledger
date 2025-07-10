#!/bin/bash

# Thai Energy Trading System - Docker Compose Management Script

set -e

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

# Function to show help
show_help() {
    echo "Thai Energy Trading System - Docker Compose Management"
    echo ""
    echo "Usage: $0 [COMMAND]"
    echo ""
    echo "Commands:"
    echo "  up          Start all services"
    echo "  down        Stop all services"
    echo "  build       Build the application image"
    echo "  logs        Show logs for all services"
    echo "  status      Show status of all services"
    echo "  restart     Restart all services"
    echo "  clean       Clean up all containers and volumes"
    echo "  health      Check health of all services"
    echo "  help        Show this help message"
    echo ""
    echo "Examples:"
    echo "  $0 up                # Start all services"
    echo "  $0 logs thai-energy-trading  # Show logs for main service"
    echo "  $0 down              # Stop all services"
}

# Function to check if docker-compose is installed
check_docker_compose() {
    if ! command -v docker-compose &> /dev/null; then
        if ! command -v docker &> /dev/null; then
            print_error "Docker is not installed. Please install Docker first."
            exit 1
        fi
        
        # Check if Docker Compose plugin is available
        if ! docker compose version &> /dev/null; then
            print_error "Docker Compose is not installed. Please install Docker Compose first."
            exit 1
        fi
        
        # Use Docker Compose plugin
        DOCKER_COMPOSE_CMD="docker compose"
    else
        # Use standalone docker-compose
        DOCKER_COMPOSE_CMD="docker-compose"
    fi
}

# Function to start services
start_services() {
    print_status "Starting Thai Energy Trading System..."
    
    # Create necessary directories
    mkdir -p docker/ssl
    mkdir -p docker/grafana/provisioning/dashboards
    
    # Generate self-signed certificate for development
    if [ ! -f docker/ssl/cert.pem ]; then
        print_status "Generating self-signed SSL certificate..."
        openssl req -x509 -newkey rsa:4096 -keyout docker/ssl/key.pem -out docker/ssl/cert.pem -days 365 -nodes \
            -subj "/C=TH/ST=Bangkok/L=Bangkok/O=Thai Energy Trading/CN=localhost" 2>/dev/null || true
    fi
    
    # Start services
    $DOCKER_COMPOSE_CMD up -d
    
    print_success "Services started successfully!"
    print_status "Access points:"
    echo "  - API Server: http://localhost:8080"
    echo "  - Grafana: http://localhost:3000 (admin/admin123)"
    echo "  - Prometheus: http://localhost:9090"
    echo "  - Nginx: http://localhost:80"
}

# Function to stop services
stop_services() {
    print_status "Stopping Thai Energy Trading System..."
    $DOCKER_COMPOSE_CMD down
    print_success "Services stopped successfully!"
}

# Function to build services
build_services() {
    print_status "Building Thai Energy Trading System..."
    $DOCKER_COMPOSE_CMD build
    print_success "Build completed successfully!"
}

# Function to show logs
show_logs() {
    if [ -n "$2" ]; then
        print_status "Showing logs for $2..."
        $DOCKER_COMPOSE_CMD logs -f "$2"
    else
        print_status "Showing logs for all services..."
        $DOCKER_COMPOSE_CMD logs -f
    fi
}

# Function to show status
show_status() {
    print_status "Service status:"
    $DOCKER_COMPOSE_CMD ps
}

# Function to restart services
restart_services() {
    print_status "Restarting Thai Energy Trading System..."
    $DOCKER_COMPOSE_CMD restart
    print_success "Services restarted successfully!"
}

# Function to clean up
clean_up() {
    print_warning "This will remove all containers and volumes. Are you sure? (y/N)"
    read -r response
    if [[ "$response" =~ ^([yY][eE][sS]|[yY])+$ ]]; then
        print_status "Cleaning up..."
        $DOCKER_COMPOSE_CMD down -v --remove-orphans
        docker system prune -f
        print_success "Cleanup completed!"
    else
        print_status "Cleanup cancelled."
    fi
}

# Function to check health
check_health() {
    print_status "Checking service health..."
    
    # Check if services are running
    if ! $DOCKER_COMPOSE_CMD ps | grep -q "Up"; then
        print_error "Services are not running. Please start them first."
        return 1
    fi
    
    # Check API health
    if curl -f http://localhost:8080/health &> /dev/null; then
        print_success "API Server: Healthy"
    else
        print_error "API Server: Unhealthy"
    fi
    
    # Check Grafana
    if curl -f http://localhost:3000/api/health &> /dev/null; then
        print_success "Grafana: Healthy"
    else
        print_error "Grafana: Unhealthy"
    fi
    
    # Check Prometheus
    if curl -f http://localhost:9090/-/healthy &> /dev/null; then
        print_success "Prometheus: Healthy"
    else
        print_error "Prometheus: Unhealthy"
    fi
    
    # Check Database
    if $DOCKER_COMPOSE_CMD exec -T postgres pg_isready -U thai_energy -d thai_energy_trading &> /dev/null; then
        print_success "PostgreSQL: Healthy"
    else
        print_error "PostgreSQL: Unhealthy"
    fi
    
    # Check Redis
    if $DOCKER_COMPOSE_CMD exec -T redis redis-cli ping | grep -q "PONG"; then
        print_success "Redis: Healthy"
    else
        print_error "Redis: Unhealthy"
    fi
}

# Main script logic
main() {
    # Check prerequisites
    check_docker_compose
    
    # Handle commands
    case "${1:-help}" in
        "up")
            start_services
            ;;
        "down")
            stop_services
            ;;
        "build")
            build_services
            ;;
        "logs")
            show_logs "$@"
            ;;
        "status")
            show_status
            ;;
        "restart")
            restart_services
            ;;
        "clean")
            clean_up
            ;;
        "health")
            check_health
            ;;
        "help"|"--help"|"-h")
            show_help
            ;;
        *)
            print_error "Unknown command: $1"
            show_help
            exit 1
            ;;
    esac
}

# Run main function
main "$@"
