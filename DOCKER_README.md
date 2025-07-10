# Thai Energy Trading System - Docker & Kubernetes Setup

This repository contains a complete Docker and Kubernetes deployment configuration for the Thai Energy Trading System, a blockchain-based decentralized energy trading platform.

## 🚀 Quick Start

### Option 1: Docker Compose (Recommended for Development)

```bash
# Build and start all services
./docker-compose.sh up

# Check service health
./docker-compose.sh health

# View logs
./docker-compose.sh logs

# Stop all services
./docker-compose.sh down
```

### Option 2: Kubernetes (Production)

```bash
# Deploy to Kubernetes
./deploy.sh

# Check deployment status
kubectl get pods -n thai-energy-trading

# Access the API
kubectl port-forward -n thai-energy-trading svc/thai-energy-api-server 8080:8080
```

## 📋 Prerequisites

- **Docker**: Version 20.10 or higher
- **Docker Compose**: Version 2.0 or higher (or Docker with Compose plugin)
- **Kubernetes**: Version 1.21 or higher (for production deployment)
- **kubectl**: Configured with cluster access
- **OpenSSL**: For generating SSL certificates

## 🏗️ Architecture

### Core Components

1. **Thai Energy Trading System**: Main blockchain application
2. **PostgreSQL**: Database for persistent storage
3. **Redis**: Caching and session management
4. **Nginx**: Load balancer and reverse proxy
5. **Prometheus**: Metrics collection
6. **Grafana**: Monitoring dashboards
7. **Oracle Service**: External data integration

### Network Architecture

```
Internet → Nginx (Port 80/443) → API Server (Port 8080)
                                ↓
                           Blockchain Network (Port 30333)
                                ↓
                        WebSocket RPC (Port 9944)
```

## 🐳 Docker Configuration

### Images

- **Base Image**: `rust:1.75-bullseye` (builder), `debian:bullseye-slim` (runtime)
- **Multi-stage build**: Optimized for size and security
- **Non-root user**: Enhanced security posture

### Volumes

- `thai_energy_data`: Blockchain data
- `thai_energy_logs`: Application logs
- `postgres_data`: Database storage
- `redis_data`: Cache storage
- `prometheus_data`: Metrics storage
- `grafana_data`: Dashboard configurations

### Environment Variables

| Variable | Description | Default |
|----------|-------------|---------|
| `RUST_LOG` | Logging level | `info` |
| `DATABASE_PASSWORD` | PostgreSQL password | `password123` |
| `JWT_SECRET` | JWT signing secret | `your-jwt-secret-here` |
| `GRID_OPERATOR_KEY` | Grid operator key | `your-grid-operator-key-here` |

## ☸️ Kubernetes Configuration

### Namespaces

- `thai-energy-trading`: Main application namespace

### Deployments

1. **Validator StatefulSet**: Blockchain validators (3 replicas)
2. **API Server Deployment**: REST API service (3 replicas)
3. **Oracle Deployment**: External data service (2 replicas)

### Services

- **LoadBalancer**: External access
- **ClusterIP**: Internal communication
- **Headless**: StatefulSet networking

### Storage

- **PersistentVolumes**: Blockchain data storage
- **ConfigMaps**: Configuration management
- **Secrets**: Sensitive data

## 🔧 Management Scripts

### Docker Compose Script

```bash
./docker-compose.sh [COMMAND]

Commands:
  up          Start all services
  down        Stop all services
  build       Build the application image
  logs        Show logs for all services
  status      Show status of all services
  restart     Restart all services
  clean       Clean up all containers and volumes
  health      Check health of all services
  help        Show help message
```

### Kubernetes Deployment Script

```bash
./deploy.sh [OPTIONS]

Options:
  --registry    Docker registry (default: your-registry.com)
  --tag         Image tag (default: latest)
  --rollback    Rollback to previous version
  --cleanup     Clean up all resources
```

## 📊 Monitoring

### Prometheus Metrics

- **Blockchain Metrics**: Block height, transaction count, finality lag
- **API Metrics**: Request rate, response time, error rate
- **System Metrics**: CPU, memory, disk usage, network I/O

### Grafana Dashboards

Access Grafana at `http://localhost:3000` (admin/admin123)

- **System Overview**: High-level system health
- **Blockchain Dashboard**: Block production and finality
- **API Performance**: Request/response metrics
- **Resource Usage**: Infrastructure metrics

### Health Checks

```bash
# Check all services
./docker-compose.sh health

# Individual service health
curl http://localhost:8080/health
curl http://localhost:3000/api/health
curl http://localhost:9090/-/healthy
```

## 🔐 Security

### Docker Security

- **Non-root containers**: All services run as non-root users
- **Minimal base images**: Reduced attack surface
- **Multi-stage builds**: No development tools in production
- **Security scanning**: Regular vulnerability scans

### Kubernetes Security

- **RBAC**: Role-based access control
- **Network Policies**: Pod-to-pod communication rules
- **Secrets Management**: Encrypted storage of sensitive data
- **Security Contexts**: Container security constraints

### SSL/TLS

- **Development**: Self-signed certificates
- **Production**: Let's Encrypt or corporate certificates
- **Mutual TLS**: Service-to-service encryption

## 📈 Performance Tuning

### Resource Allocation

| Service | CPU Request | CPU Limit | Memory Request | Memory Limit |
|---------|-------------|-----------|----------------|--------------|
| API Server | 500m | 1000m | 512Mi | 1Gi |
| Validator | 1000m | 2000m | 1Gi | 2Gi |
| Oracle | 200m | 500m | 256Mi | 512Mi |
| Database | 500m | 1000m | 1Gi | 2Gi |

### Scaling

```bash
# Scale API servers
kubectl scale deployment thai-energy-api-server --replicas=5 -n thai-energy-trading

# Scale Oracle services
kubectl scale deployment thai-energy-oracle --replicas=3 -n thai-energy-trading
```

### Horizontal Pod Autoscaler

```yaml
apiVersion: autoscaling/v2
kind: HorizontalPodAutoscaler
metadata:
  name: thai-energy-api-hpa
spec:
  scaleTargetRef:
    apiVersion: apps/v1
    kind: Deployment
    name: thai-energy-api-server
  minReplicas: 3
  maxReplicas: 10
  metrics:
  - type: Resource
    resource:
      name: cpu
      target:
        type: Utilization
        averageUtilization: 70
```

## 🔄 CI/CD Pipeline

### GitHub Actions

```yaml
name: Build and Deploy

on:
  push:
    branches: [ main ]
  pull_request:
    branches: [ main ]

jobs:
  build:
    runs-on: ubuntu-latest
    steps:
    - uses: actions/checkout@v3
    - name: Build Docker image
      run: docker build -t thai-energy-trading:${{ github.sha }} .
    - name: Run tests
      run: docker run --rm thai-energy-trading:${{ github.sha }} cargo test
    - name: Push to registry
      run: |
        echo ${{ secrets.DOCKER_PASSWORD }} | docker login -u ${{ secrets.DOCKER_USERNAME }} --password-stdin
        docker push thai-energy-trading:${{ github.sha }}
```

### GitLab CI

```yaml
stages:
  - build
  - test
  - deploy

variables:
  DOCKER_IMAGE: $CI_REGISTRY_IMAGE:$CI_COMMIT_SHA

build:
  stage: build
  script:
    - docker build -t $DOCKER_IMAGE .
    - docker push $DOCKER_IMAGE

test:
  stage: test
  script:
    - docker run --rm $DOCKER_IMAGE cargo test

deploy:
  stage: deploy
  script:
    - ./deploy.sh --registry $CI_REGISTRY --tag $CI_COMMIT_SHA
  only:
    - main
```

## 🛠️ Troubleshooting

### Common Issues

1. **Port Conflicts**
   ```bash
   # Check if ports are in use
   lsof -i :8080
   lsof -i :3000
   lsof -i :9090
   ```

2. **Permission Issues**
   ```bash
   # Fix file permissions
   sudo chown -R $USER:$USER docker/
   chmod +x docker-compose.sh deploy.sh
   ```

3. **Database Connection**
   ```bash
   # Check PostgreSQL logs
   ./docker-compose.sh logs postgres
   
   # Connect to database
   docker-compose exec postgres psql -U thai_energy -d thai_energy_trading
   ```

4. **Memory Issues**
   ```bash
   # Increase Docker memory limit
   # Docker Desktop: Settings → Resources → Advanced → Memory
   
   # Check container memory usage
   docker stats
   ```

### Debugging Commands

```bash
# Check service logs
./docker-compose.sh logs thai-energy-trading

# Execute shell in container
docker-compose exec thai-energy-trading /bin/bash

# Check network connectivity
docker-compose exec thai-energy-trading ping postgres

# Inspect container
docker inspect thai-energy-trading

# Check disk usage
docker system df
```

## 📚 Documentation

- **API Documentation**: `API_DOCUMENTATION.md`
- **Architecture Guide**: `MODERN_ARCHITECTURE.md`
- **Network Topology**: `NETWORK_TOPOLOGY.md`
- **Deployment Guide**: `DOCKER_K8S_DEPLOYMENT_GUIDE.md`

## 🤝 Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes
4. Add tests
5. Submit a pull request

## 📄 License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## 🆘 Support

- **Documentation**: https://docs.thai-energy-trading.com
- **Issues**: https://github.com/your-org/thai-energy-trading/issues
- **Discussions**: https://github.com/your-org/thai-energy-trading/discussions
- **Email**: support@thai-energy-trading.com

---

**Note**: This is a demonstration system for educational purposes. Please ensure proper security measures are in place before using in production environments.
