# Docker & Kubernetes Deployment Guide

## Table of Contents
1. [Overview](#overview)
2. [Prerequisites](#prerequisites)
3. [Local Development with Docker Compose](#local-development)
4. [Production Deployment with Kubernetes](#kubernetes-deployment)
5. [Configuration Management](#configuration-management)
6. [Monitoring and Logging](#monitoring-and-logging)
7. [Troubleshooting](#troubleshooting)

## Overview

This guide provides comprehensive instructions for deploying the Thai Energy Trading System using Docker and Kubernetes. The system includes:

- **Thai Energy Trading System**: Main blockchain application
- **PostgreSQL Database**: Persistent data storage
- **Redis Cache**: Session and caching layer
- **Nginx**: Load balancer and reverse proxy
- **Prometheus**: Metrics collection
- **Grafana**: Monitoring dashboards
- **Oracle Service**: External data integration

## Prerequisites

### For Local Development
- Docker Desktop or Docker Engine (>= 20.10)
- Docker Compose (>= 2.0)
- At least 4GB RAM available for Docker
- At least 10GB free disk space

### For Production Deployment
- Kubernetes cluster (>= 1.20)
- kubectl configured to access your cluster
- Helm (>= 3.0) - optional but recommended
- Container registry access (Docker Hub, GCR, ACR, etc.)

## Local Development

### Quick Start

1. **Clone and navigate to the project:**
   ```bash
   cd /path/to/thai-energy-trading
   ```

2. **Build and start all services:**
   ```bash
   ./docker-compose.sh up
   ```

3. **Access the application:**
   - API Server: http://localhost:8080
   - Grafana Dashboard: http://localhost:3000 (admin/admin123)
   - Prometheus: http://localhost:9090

### Management Commands

The `docker-compose.sh` script provides convenient management commands:

```bash
# Build all services
./docker-compose.sh build

# Start all services
./docker-compose.sh up

# Start services in background
./docker-compose.sh up -d

# Stop all services
./docker-compose.sh down

# View logs
./docker-compose.sh logs

# View logs for specific service
./docker-compose.sh logs thai-energy-trading

# Execute commands in containers
./docker-compose.sh exec thai-energy-trading bash

# Scale services
./docker-compose.sh scale thai-energy-trading=3
```

### Development Workflow

1. **Code Changes**: Make changes to your source code
2. **Rebuild**: Run `./docker-compose.sh build` to rebuild containers
3. **Restart**: Run `./docker-compose.sh restart thai-energy-trading`
4. **Test**: Access the application and test your changes

### Service Configuration

#### Environment Variables
Key environment variables for local development:

```bash
# Database Configuration
DATABASE_PASSWORD=password123

# Security
JWT_SECRET=your-jwt-secret-here
GRID_OPERATOR_KEY=your-grid-operator-key-here

# Logging
RUST_LOG=info

# Oracle Configuration
ORACLE_API_KEY=your-oracle-api-key
```

#### Volumes
- `thai_energy_data`: Application data persistence
- `postgres_data`: Database data persistence
- `redis_data`: Redis data persistence
- `prometheus_data`: Metrics data
- `grafana_data`: Dashboard configurations

## Kubernetes Deployment

### Cluster Setup

1. **Create namespace:**
   ```bash
   kubectl create namespace thai-energy-trading
   ```

2. **Deploy services:**
   ```bash
   kubectl apply -f k8s/
   ```

### Key Kubernetes Resources

#### ConfigMaps
- `thai-energy-config`: Application configuration
- `prometheus-config`: Prometheus configuration
- `grafana-config`: Grafana dashboards

#### Secrets
- `thai-energy-secrets`: Database passwords, JWT secrets
- `tls-secrets`: TLS certificates

#### Deployments
- `thai-energy-trading`: Main application (3 replicas)
- `postgres`: Database (1 replica with persistent storage)
- `redis`: Cache (1 replica)
- `prometheus`: Metrics (1 replica)
- `grafana`: Monitoring (1 replica)

#### Services
- `thai-energy-trading-service`: LoadBalancer for main app
- `postgres-service`: ClusterIP for database
- `redis-service`: ClusterIP for cache
- `prometheus-service`: ClusterIP for metrics
- `grafana-service`: LoadBalancer for dashboards

### Ingress Configuration

```yaml
apiVersion: networking.k8s.io/v1
kind: Ingress
metadata:
  name: thai-energy-ingress
  namespace: thai-energy-trading
spec:
  rules:
  - host: thai-energy-trading.example.com
    http:
      paths:
      - path: /
        pathType: Prefix
        backend:
          service:
            name: thai-energy-trading-service
            port:
              number: 8080
```

### Persistent Storage

The system uses persistent volumes for:
- PostgreSQL data (`postgres-pvc`: 20Gi)
- Prometheus metrics (`prometheus-pvc`: 10Gi)
- Application logs (`thai-energy-logs-pvc`: 5Gi)

## Configuration Management

### Application Configuration

Configuration is managed through:
1. **Environment variables**: Runtime configuration
2. **ConfigMaps**: Application settings
3. **Secrets**: Sensitive data

### Key Configuration Files

- `docker/config/app.toml`: Main application configuration
- `docker/prometheus.yml`: Prometheus scraping configuration
- `docker/nginx.conf`: Nginx proxy configuration

### Security Configuration

```toml
[security]
jwt_secret = "your-jwt-secret-here"
grid_operator_key = "your-grid-operator-key-here"
encryption_key = "your-encryption-key-here"

[database]
host = "postgres"
port = 5432
database = "thai_energy_trading"
username = "thai_energy"
password = "password123"
```

## Monitoring and Logging

### Prometheus Metrics

The system exposes metrics on `/metrics` endpoint:
- Transaction throughput
- Energy trading volumes
- System performance metrics
- Database connection health

### Grafana Dashboards

Pre-configured dashboards include:
- **System Overview**: High-level system metrics
- **Energy Trading**: Trading-specific metrics
- **Database Performance**: PostgreSQL metrics
- **Network Health**: P2P network status

### Log Aggregation

Logs are collected from:
- Application logs: `/app/logs/`
- Database logs: PostgreSQL logs
- System logs: Docker/Kubernetes logs

## Troubleshooting

### Common Issues

#### Build Failures
```bash
# Clean Docker cache
docker system prune -a

# Rebuild with no cache
./docker-compose.sh build --no-cache
```

#### Connection Issues
```bash
# Check service status
./docker-compose.sh ps

# Check logs
./docker-compose.sh logs thai-energy-trading

# Check network connectivity
./docker-compose.sh exec thai-energy-trading ping postgres
```

#### Performance Issues
```bash
# Check resource usage
docker stats

# Scale services
./docker-compose.sh scale thai-energy-trading=3
```

### Health Checks

All services include health checks:
- **Application**: `GET /health`
- **Database**: `pg_isready`
- **Redis**: `redis-cli ping`
- **Prometheus**: `GET /-/healthy`

### Database Maintenance

```bash
# Backup database
./docker-compose.sh exec postgres pg_dump -U thai_energy thai_energy_trading > backup.sql

# Restore database
./docker-compose.sh exec -i postgres psql -U thai_energy thai_energy_trading < backup.sql
```

## Production Considerations

### Security Hardening
- Use secrets management (HashiCorp Vault, Kubernetes secrets)
- Enable TLS for all communications
- Implement network policies
- Regular security updates

### Performance Optimization
- Use production-grade PostgreSQL configuration
- Implement connection pooling
- Configure appropriate resource limits
- Use horizontal pod autoscaling

### Backup Strategy
- Regular database backups
- Configuration backups
- Persistent volume snapshots
- Disaster recovery procedures

### Monitoring and Alerting
- Set up alerting rules in Prometheus
- Configure notification channels
- Implement log aggregation
- Regular health checks

## Support and Maintenance

### Regular Tasks
- Monitor system health
- Update container images
- Backup data
- Review security logs
- Performance optimization

### Updates and Upgrades
- Test updates in staging environment
- Use rolling updates for zero-downtime deployments
- Maintain rollback procedures
- Document changes

For additional support, please refer to the project documentation or contact the development team.
