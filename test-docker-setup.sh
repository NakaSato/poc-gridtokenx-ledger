#!/bin/bash
# Test script for Docker setup validation

set -e

echo "🔍 Thai Energy Trading System - Docker Setup Validation"
echo "========================================================"

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Test functions
test_docker_installed() {
    echo -n "Testing Docker installation... "
    if command -v docker &> /dev/null; then
        echo -e "${GREEN}✓ Docker is installed${NC}"
        docker --version
    else
        echo -e "${RED}✗ Docker is not installed${NC}"
        exit 1
    fi
}

test_docker_compose_installed() {
    echo -n "Testing Docker Compose installation... "
    if command -v docker-compose &> /dev/null || docker compose version &> /dev/null; then
        echo -e "${GREEN}✓ Docker Compose is installed${NC}"
        if command -v docker-compose &> /dev/null; then
            docker-compose --version
        else
            docker compose version
        fi
    else
        echo -e "${RED}✗ Docker Compose is not installed${NC}"
        exit 1
    fi
}

test_docker_running() {
    echo -n "Testing Docker daemon... "
    if docker info &> /dev/null; then
        echo -e "${GREEN}✓ Docker daemon is running${NC}"
    else
        echo -e "${RED}✗ Docker daemon is not running${NC}"
        exit 1
    fi
}

test_project_files() {
    echo -n "Testing project files... "
    required_files=(
        "Dockerfile"
        "docker-compose.yml"
        "docker-compose.sh"
        "Cargo.toml"
        "src/main.rs"
        "benches/performance_benchmark.rs"
    )
    
    missing_files=()
    for file in "${required_files[@]}"; do
        if [ ! -f "$file" ]; then
            missing_files+=("$file")
        fi
    done
    
    if [ ${#missing_files[@]} -eq 0 ]; then
        echo -e "${GREEN}✓ All required files are present${NC}"
    else
        echo -e "${RED}✗ Missing files: ${missing_files[*]}${NC}"
        exit 1
    fi
}

test_docker_build() {
    echo -n "Testing Docker build... "
    if ./docker-compose.sh build > /tmp/docker_build.log 2>&1; then
        echo -e "${GREEN}✓ Docker build successful${NC}"
    else
        echo -e "${RED}✗ Docker build failed${NC}"
        echo "Build log (last 20 lines):"
        tail -n 20 /tmp/docker_build.log
        exit 1
    fi
}

test_services_start() {
    echo -n "Testing services startup... "
    if ./docker-compose.sh up -d > /tmp/docker_up.log 2>&1; then
        echo -e "${GREEN}✓ Services started${NC}"
        
        # Wait for services to be ready
        echo "Waiting for services to be ready..."
        sleep 10
        
        # Check service health
        services=("thai-energy-trading" "postgres" "redis")
        for service in "${services[@]}"; do
            if docker ps --filter "name=$service" --filter "status=running" | grep -q "$service"; then
                echo -e "${GREEN}✓ $service is running${NC}"
            else
                echo -e "${RED}✗ $service is not running${NC}"
                docker-compose logs "$service" | tail -10
            fi
        done
    else
        echo -e "${RED}✗ Failed to start services${NC}"
        cat /tmp/docker_up.log
        exit 1
    fi
}

test_api_endpoints() {
    echo "Testing API endpoints..."
    
    # Wait for the application to be ready
    echo "Waiting for API to be ready..."
    for i in {1..30}; do
        if curl -s -f http://localhost:8080/health > /dev/null 2>&1; then
            echo -e "${GREEN}✓ API is responding${NC}"
            break
        fi
        if [ $i -eq 30 ]; then
            echo -e "${RED}✗ API did not respond within timeout${NC}"
            exit 1
        fi
        sleep 2
    done
    
    # Test specific endpoints
    endpoints=(
        "http://localhost:8080/health"
        "http://localhost:8080/api/status"
    )
    
    for endpoint in "${endpoints[@]}"; do
        echo -n "Testing $endpoint... "
        if curl -s -f "$endpoint" > /dev/null; then
            echo -e "${GREEN}✓ OK${NC}"
        else
            echo -e "${YELLOW}⚠ Not available (may be expected)${NC}"
        fi
    done
}

test_database_connection() {
    echo -n "Testing database connection... "
    if ./docker-compose.sh exec -T postgres psql -U thai_energy -d thai_energy_trading -c "SELECT 1;" > /dev/null 2>&1; then
        echo -e "${GREEN}✓ Database connection successful${NC}"
    else
        echo -e "${RED}✗ Database connection failed${NC}"
        exit 1
    fi
}

test_redis_connection() {
    echo -n "Testing Redis connection... "
    if ./docker-compose.sh exec -T redis redis-cli ping | grep -q "PONG"; then
        echo -e "${GREEN}✓ Redis connection successful${NC}"
    else
        echo -e "${RED}✗ Redis connection failed${NC}"
        exit 1
    fi
}

cleanup() {
    echo "Cleaning up..."
    ./docker-compose.sh down -v > /dev/null 2>&1 || true
}

# Main test execution
main() {
    echo "Starting Docker setup validation..."
    echo
    
    test_docker_installed
    test_docker_compose_installed
    test_docker_running
    test_project_files
    
    echo
    echo "🏗️  Testing Docker Build Process"
    echo "--------------------------------"
    test_docker_build
    
    echo
    echo "🚀 Testing Services"
    echo "-------------------"
    test_services_start
    
    echo
    echo "🔌 Testing Connectivity"
    echo "----------------------"
    test_api_endpoints
    test_database_connection
    test_redis_connection
    
    echo
    echo -e "${GREEN}🎉 All tests passed! Docker setup is working correctly.${NC}"
    echo
    echo "📋 Next steps:"
    echo "- Access the application: http://localhost:8080"
    echo "- View Grafana dashboards: http://localhost:3000"
    echo "- Check Prometheus metrics: http://localhost:9090"
    echo "- View logs: ./docker-compose.sh logs"
    echo "- Stop services: ./docker-compose.sh down"
}

# Handle cleanup on exit
trap cleanup EXIT

# Run main function
main "$@"
