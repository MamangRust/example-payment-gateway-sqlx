#!/bin/bash

# Docker Build Script for Payment Gateway Services
# This script builds Docker images for all service

set -e

echo "🐳 Building Docker images for Payment Gateway services..."

# Colors for output
RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m' # No Color

# Function to print colored output
print_status() {
    echo -e "${GREEN}[INFO]${NC} $1"
}

print_warning() {
    echo -e "${YELLOW}[WARNING]${NC} $1"
}

print_error() {
    echo -e "${RED}[ERROR]${NC} $1"
}

# Check if docker is installed and running
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        print_error "Docker is not running. Please start Docker."
        exit 1
    fi
    print_status "Docker is installed and running"
}

# Build Docker image for a service
build_service_image() {
    local service=$1
    local dockerfile="Dockerfile.${service}"
    
    print_status "Building ${service} service image..."
    
    if [ ! -f "$dockerfile" ]; then
        print_error "Dockerfile $dockerfile not found"
        return 1
    fi
    
    # Build the image
    if docker build -f "$dockerfile" -t "payment-gateway-sqlx/${service}:latest" .; then
        print_status "✅ Successfully built ${service}:latest"
        return 0
    else
        print_error "❌ Failed to build ${service}:latest"
        return 1
    fi
}

# Build all service images
build_all_images() {
    print_status "Building all service images..."
    
    # List of services to build
    services=("auth" "user" "card" "merchant" "role" "saldo" "transaction" "topup" "transfer" "withdraw" "apigateway")
    
    local failed_builds=0
    
    for service in "${services[@]}"; do
        if ! build_service_image "$service"; then
            ((failed_builds++))
        fi
    done
    
    if [ $failed_builds -eq 0 ]; then
        print_status "🎉 All images built successfully!"
    else
        print_warning "⚠️  $failed_builds images failed to build"
        return 1
    fi
}

# Show built images
show_built_images() {
    print_status "Built Docker images:"
    echo ""
    docker images | grep -E "(auth|user|card|merchant|role|saldo|transaction|topup|transfer|withdraw|apigateway)" | head -20
    echo ""
}

# Cleanup function
cleanup() {
    print_status "Build process completed"
}

# Main execution
main() {
    # Set trap for cleanup
    trap cleanup EXIT
    
    # Run checks
    check_docker
    
    # Build all images
    build_all_images
    
    # Show built images
    show_built_images
    
    print_status "Docker build process completed! 🎉"
}

# Handle script interruption
trap 'print_error "Build interrupted"; exit 1' INT

# Run main function
main "$@"