#!/bin/bash

# Payment Gateway Minikube Setup Script
# This script sets up the complete payment gateway infrastructure on Minikube

set -e

echo "🚀 Starting Payment Gateway Minikube Setup..."

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

# Check if minikube is installed
check_minikube() {
    if ! command -v minikube &> /dev/null; then
        print_error "Minikube is not installed. Please install Minikube first."
        echo "Visit: https://minikube.sigs.k8s.io/docs/start/"
        exit 1
    fi
    print_status "Minikube is installed"
}

# Check if kubectl is installed
check_kubectl() {
    if ! command -v kubectl &> /dev/null; then
        print_error "kubectl is not installed. Please install kubectl first."
        echo "Visit: https://kubernetes.io/docs/tasks/tools/"
        exit 1
    fi
    print_status "kubectl is installed"
}

# Check if docker is installed and running
check_docker() {
    if ! command -v docker &> /dev/null; then
        print_error "Docker is not installed. Please install Docker first."
        echo "Visit: https://docs.docker.com/get-docker/"
        exit 1
    fi
    
    if ! docker info &> /dev/null; then
        print_error "Docker is not running. Please start Docker."
        exit 1
    fi
    print_status "Docker is installed and running"
}

# Start Minikube
start_minikube() {
    print_status "Starting Minikube..."
    
    # Check if minikube is already running
    if minikube status | grep -q "Running"; then
        print_warning "Minikube is already running"
    else
        minikube start --cpus=8 --memory=16384 --disk-size=40g
        print_status "Minikube started successfully"
    fi
    
    # Enable addons
    print_status "Enabling Minikube addons..."
    minikube addons enable ingress
    minikube addons enable metrics-server
    minikube addons enable dashboard
}

# Load pre-built Docker images into Minikube
load_docker_images() {
    print_status "Loading pre-built Docker images into Minikube..."

    # Get minikube docker environment
    eval $(minikube docker-env)

    # Load all microservice images
    services=("auth" "user" "card" "merchant" "role" "saldo" "transaction" "topup" "transfer" "withdraw" "apigateway")

    for service in "${services[@]}"; do
        img="payment-gateway-sqlx/${service}:latest"
        print_status "Loading $img into Minikube..."    

        # Load into Minikube
        if minikube image load "$img" > /dev/null 2>&1; then
            print_status "$img loaded successfully"
        else
            print_warning "Could not load $img into Minikube"
        fi
    done
}

# Apply Kubernetes manifests
apply_k8s_manifests() {
    print_status "Applying Kubernetes manifests..."

    # Apply namespace first
    kubectl apply -f k8s/namespace.yaml

    # Apply database services
    print_status "Deploying database services..."
    kubectl apply -f k8s/database/

    # Wait for database to be ready
    print_status "Waiting for database services to be ready..."
    kubectl wait --for=condition=ready pod -l app=postgres -n payment-gateway --timeout=300s
    kubectl wait --for=condition=ready pod -l app=pgbouncer -n payment-gateway --timeout=300s
    kubectl wait --for=condition=ready pod -l app=redis -n payment-gateway --timeout=300s

    # Apply observability stack
    print_status "Deploying observability stack..."
    kubectl apply -f k8s/observability/

    # Wait for core observability services to be ready
    print_status "Waiting for observability services to be ready..."
    kubectl wait --for=condition=ready pod -l app=jaeger -n payment-gateway --timeout=300s || print_warning "Jaeger may not be ready yet"
    kubectl wait --for=condition=ready pod -l app=prometheus -n payment-gateway --timeout=300s || print_warning "Prometheus may not be ready yet"
    kubectl wait --for=condition=ready pod -l app=grafana -n payment-gateway --timeout=300s || print_warning "Grafana may not be ready yet"
    kubectl wait --for=condition=ready pod -l app=otel-collector -n payment-gateway --timeout=300s || print_warning "OTEL Collector may not be ready yet"

    # Apply microservices
    print_status "Deploying microservices..."
    kubectl apply -f k8s/microservices/

    # Wait for microservices to be ready
    print_status "Waiting for microservices to be ready..."
    services=("auth" "user" "card" "merchant" "role" "saldo" "transaction" "topup" "transfer" "withdraw")
    for service in "${services[@]}"; do
        kubectl wait --for=condition=ready pod -l app=$service -n payment-gateway --timeout=300s || print_warning "$service may not be ready yet"
    done

    # Apply gateway services
    print_status "Deploying gateway services..."
    kubectl apply -f k8s/gateway/

    # Wait for gateway to be ready
    print_status "Waiting for gateway services to be ready..."
    kubectl wait --for=condition=ready pod -l app=apigateway -n payment-gateway --timeout=300s || print_warning "API Gateway may not be ready yet"
    kubectl wait --for=condition=ready pod -l app=nginx -n payment-gateway --timeout=300s || print_warning "NGINX may not be ready yet"
}

# Show access information
show_access_info() {
    print_status "Getting access information..."
    
    echo ""
    echo "🎉 Payment Gateway is now running on Minikube!"
    echo ""
    echo "📊 Access URLs:"
    echo "=================================="
    
    # Get Minikube IP
    MINIKUBE_IP=$(minikube ip)
    
    echo "🌐 Main Application:"
    echo "   URL: http://$MINIKUBE_IP:30080"
    echo ""
    
    echo "📈 Observability Stack:"
    echo "   Grafana:      http://$MINIKUBE_IP:30030 (admin/admin)"
    echo "   Prometheus:   http://$MINIKUBE_IP:30090"
    echo "   Jaeger:       http://$MINIKUBE_IP:31686"
    echo "   Loki:         http://$MINIKUBE_IP:30010"
    echo "   Alertmanager: http://$MINIKUBE_IP:30093"
    echo ""
    
    echo "🔧 Kubernetes Dashboard:"
    echo "   Run: minikube dashboard"
    echo ""
    
    echo "📝 Useful Commands:"
    echo "=================================="
    echo "View all pods:     kubectl get pods -n payment-gateway"
    echo "View services:      kubectl get services -n payment-gateway"
    echo "View logs:         kubectl logs -f deployment/<service-name> -n payment-gateway"
    echo "Access pod shell:  kubectl exec -it <pod-name> -n payment-gateway -- /bin/bash"
    echo ""
    
    echo "🗂️  Log Files Location:"
    echo "   Application logs are mounted to /var/log/app in each container"
    echo ""
}

# Cleanup function
cleanup() {
    print_status "Cleaning up..."
    # Add any cleanup logic here if needed
}

# Main execution
main() {
    # Set trap for cleanup
    trap cleanup EXIT
    
    # Run checks
    check_minikube
    check_kubectl
    check_docker
    
    # Start Minikube
    start_minikube
    
    # Load Docker images
    load_docker_images
    
    # Apply Kubernetes manifests
    apply_k8s_manifests
    
    # Show access information
    show_access_info
    
    print_status "Setup completed successfully! 🎉"
}

# Handle script interruption
trap 'print_error "Setup interrupted"; exit 1' INT

# Run main function
main "$@"