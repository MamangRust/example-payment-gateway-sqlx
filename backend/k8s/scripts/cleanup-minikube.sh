#!/bin/bash
set -e

RED='\033[0;31m'
GREEN='\033[0;32m'
YELLOW='\033[1;33m'
NC='\033[0m'

print_status() { echo -e "${GREEN}[INFO]${NC} $1"; }
print_warning() { echo -e "${YELLOW}[WARNING]${NC} $1"; }
print_error() { echo -e "${RED}[ERROR]${NC} $1"; }

echo "🧹 Cleaning up Payment Gateway from Minikube..."

if ! minikube status | grep -q "Running"; then
    print_warning "Minikube is not running"
    exit 0
fi

print_status "Deleting payment-gateway namespace..."
kubectl delete namespace payment-gateway --ignore-not-found=true

read -p "Do you want to stop Minikube? (y/N): " -n 1 -r
echo
if [[ $REPLY =~ ^[Yy]$ ]]; then
    print_status "Stopping Minikube..."
    minikube stop
    minikube delete
fi

print_status "Cleanup completed! 🧹"
echo "💀 Removed all traces of the minikube cluster."
