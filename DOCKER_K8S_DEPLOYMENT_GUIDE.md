# Thai Energy Trading System - Docker & Kubernetes Deployment Guide

## Overview

This guide provides comprehensive instructions for deploying the Thai Energy Trading System using Docker and Kubernetes. The system is designed with a microservices architecture and supports both development and production deployments.

## Architecture

The system consists of the following components:

1. **Validator Nodes** - Core blockchain validators
2. **API Server** - REST API for external applications
3. **Oracle Service** - External data integration
4. **Monitoring Stack** - Prometheus, Grafana, and alerting
5. **Load Balancer** - Ingress controller for external access

## Prerequisites

- Docker (>= 20.10)
- Kubernetes cluster (>= 1.21)
- kubectl configured with cluster access
- Helm (>= 3.0) - optional for monitoring stack

## Quick Start

### 1. Build and Run with Docker

```bash
# Build the Docker image
docker build -t thai-energy-trading:latest .

# Run locally
docker run -p 8080:8080 -p 9944:9944 -p 30333:30333 thai-energy-trading:latest
```

### 2. Deploy to Kubernetes

```bash
# Make the deployment script executable
chmod +x deploy.sh

# Deploy the entire system
./deploy.sh

# Or deploy manually
kubectl apply -f k8s/01-namespace-configmap.yaml
kubectl apply -f k8s/02-secrets-storage.yaml
kubectl apply -f k8s/03-validator-statefulset.yaml
kubectl apply -f k8s/04-api-server-deployment.yaml
kubectl apply -f k8s/05-oracle-deployment.yaml
kubectl apply -f k8s/06-ingress-loadbalancer.yaml
kubectl apply -f k8s/07-rbac-security.yaml
kubectl apply -f k8s/08-monitoring.yaml
kubectl apply -f k8s/09-autoscaling-limits.yaml
```

## Docker Configuration

### Multi-Stage Dockerfile

The Dockerfile uses a multi-stage build for optimal image size and security:

- **Builder Stage**: Compiles Rust application with all dependencies
- **Runtime Stage**: Minimal Debian image with only runtime dependencies

Key features:
- Non-root user for security
- Health checks for container orchestration
- Optimized layer caching
- Minimal attack surface

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level | `info` |
| `DATABASE_PASSWORD` | Database password | From secret |
| `JWT_SECRET` | JWT signing secret | From secret |
| `GRID_OPERATOR_KEY` | Grid operator private key | From secret |

## Kubernetes Configuration

### Namespace and Configuration

- **Namespace**: `thai-energy-trading`
- **ConfigMap**: System configuration in TOML format
- **Secrets**: Sensitive data like passwords and keys

### Core Components

#### 1. Validator Nodes (StatefulSet)

```yaml
apiVersion: apps/v1
kind: StatefulSet
metadata:
  name: thai-energy-validator
```

Features:
- Persistent storage for blockchain data
- Ordered deployment and scaling
- Stable network identities
- Automatic peer discovery

#### 2. API Server (Deployment)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: thai-energy-api-server
```

Features:
- Multiple replicas for high availability
- Rolling updates for zero-downtime deployments
- Health checks and readiness probes
- Horizontal Pod Autoscaling

#### 3. Oracle Service (Deployment)

```yaml
apiVersion: apps/v1
kind: Deployment
metadata:
  name: thai-energy-oracle
```

Features:
- External data integration
- Configurable data sources
- Retry mechanisms
- Rate limiting

### Networking

#### Services

1. **Validator Headless Service**: Internal validator communication
2. **API Server Service**: External API access
3. **Oracle Service**: Internal oracle communication

#### Ingress

- **LoadBalancer**: External access to API server
- **SSL Termination**: HTTPS support
- **Rate Limiting**: Protection against abuse

### Security

#### RBAC (Role-Based Access Control)

- **ServiceAccount**: Dedicated service accounts
- **ClusterRole**: Minimal required permissions
- **NetworkPolicy**: Pod-to-pod communication rules

#### Secrets Management

- **Database passwords**: Stored in Kubernetes secrets
- **API keys**: Encrypted at rest
- **TLS certificates**: Managed by cert-manager

### Monitoring and Observability

#### Metrics

- **Prometheus**: Metrics collection
- **Grafana**: Visualization dashboards
- **AlertManager**: Alert routing and management

#### Logging

- **Container logs**: Collected by Kubernetes
- **Application logs**: Structured JSON format
- **Audit logs**: Compliance tracking

### Scaling and Resource Management

#### Horizontal Pod Autoscaling (HPA)

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
```

- CPU-based scaling
- Memory-based scaling
- Custom metrics support

#### Resource Limits

- **CPU**: Request/limit per container
- **Memory**: Request/limit per container
- **Storage**: Persistent volume sizing

## Deployment Script

The `deploy.sh` script automates the entire deployment process:

```bash
#!/bin/bash
# Features:
# - Prerequisites checking
# - Docker image building
# - Kubernetes deployment
# - Health checks
# - Rollback capability
```

### Usage

```bash
# Deploy with default settings
./deploy.sh

# Deploy with custom registry
./deploy.sh --registry your-registry.com

# Deploy with specific tag
./deploy.sh --tag v1.0.0

# Rollback deployment
./deploy.sh --rollback

# Clean up resources
./deploy.sh --cleanup
```

## Configuration Management

### Environment-Specific Configurations

1. **Development**: `config/dev.toml`
2. **Staging**: `config/staging.toml`
3. **Production**: `config/prod.toml`

### Configuration Parameters

#### Network Configuration

```toml
[network]
listen_addr = "0.0.0.0:30333"
public_addr = "127.0.0.1:30333"
node_key_file = "/app/data/node.key"
chain_spec_path = "/app/config/thai_energy_chain_spec.json"
```

#### API Configuration

```toml
[api]
http_addr = "0.0.0.0:8080"
ws_addr = "0.0.0.0:9944"
cors_origins = ["*"]
max_connections = 1000
```

#### Database Configuration

```toml
[database]
path = "/app/data/db"
cache_size = "1GB"
pruning = "archive"
```

## Monitoring and Alerting

### Prometheus Metrics

- **Blockchain metrics**: Block height, transaction count
- **API metrics**: Request rate, response time
- **System metrics**: CPU, memory, disk usage

### Grafana Dashboards

1. **System Overview**: High-level system health
2. **Blockchain Metrics**: Block production, finality
3. **API Performance**: Request/response metrics
4. **Resource Usage**: CPU, memory, storage

### Alerts

- **High CPU usage**: > 80% for 5 minutes
- **Memory pressure**: > 90% for 2 minutes
- **Blockchain sync lag**: > 10 blocks behind
- **API errors**: > 5% error rate

## Troubleshooting

### Common Issues

1. **Pod not starting**: Check resource limits and quotas
2. **Connection issues**: Verify network policies
3. **Storage issues**: Check persistent volume claims
4. **Configuration errors**: Validate ConfigMap and Secrets

### Debugging Commands

```bash
# Check pod status
kubectl get pods -n thai-energy-trading

# View pod logs
kubectl logs -n thai-energy-trading deployment/thai-energy-api-server

# Check events
kubectl get events -n thai-energy-trading

# Debug networking
kubectl exec -it pod-name -- netstat -tlnp
```

## Performance Tuning

### Resource Optimization

1. **CPU**: Set appropriate requests/limits
2. **Memory**: Monitor heap usage
3. **Storage**: Use SSD for blockchain data
4. **Network**: Optimize bandwidth usage

### Scaling Guidelines

- **Validators**: 3-5 nodes for production
- **API servers**: Scale based on load
- **Oracle services**: 1-2 instances sufficient

## Security Best Practices

1. **Use non-root containers**
2. **Implement network policies**
3. **Rotate secrets regularly**
4. **Enable audit logging**
5. **Use minimal base images**
6. **Scan images for vulnerabilities**

## Backup and Recovery

### Data Backup

```bash
# Backup blockchain data
kubectl exec -n thai-energy-trading thai-energy-validator-0 -- \
  tar -czf /tmp/blockchain-backup.tar.gz /app/data/db
```

### Disaster Recovery

1. **Database restoration**: From persistent volume snapshots
2. **Configuration backup**: GitOps approach
3. **Secret recovery**: From secure key management

## Upgrades and Maintenance

### Rolling Updates

```bash
# Update image tag
kubectl set image deployment/thai-energy-api-server \
  api-server=thai-energy-trading:v1.1.0 \
  -n thai-energy-trading
```

### Maintenance Windows

- **Scheduled downtime**: Use maintenance mode
- **Emergency patches**: Rolling updates
- **Major upgrades**: Blue-green deployment

## Support and Documentation

- **Repository**: https://github.com/your-org/thai-energy-trading
- **Documentation**: https://docs.thai-energy-trading.com
- **Support**: support@thai-energy-trading.com

## License

This project is licensed under the MIT License - see the LICENSE file for details.
