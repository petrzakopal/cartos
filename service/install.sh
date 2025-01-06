#!/bin/bash

# Define service file path and the contents of the service file
BACKEND_SERVICE_FILE="/etc/systemd/system/cartos-backend.service"
FRONTEND_SERVICE_FILE="/etc/systemd/system/cartos-frontend.service"
BACKEND_SERVICE_CONTENTS="[Unit]
Description=Cartos Backend
After=multi-user.target usbutils.target pcscd.service pcscd.socket
Requires=pcscd.service pcscd.socket

[Service]
ExecStart=/cartos/backend/app
WorkingDirectory=/cartos/backend
Restart=always
User=root
Group=sudo
Environment=RUST_LOG=debug
Environment=RUST_BACKTRACE=1
Environment=NODE_ENV=production

[Install]
WantedBy=multi-user.target"

FRONTEND_SERVICE_CONTENTS="[Unit]
Description=Cartos Frontend
After=multi-user.target

[Service]
ExecStartPre=/bin/bash -c 'export USER=$(whoami)'
ExecStart=/home/$USER/.bun/bin/bun run /cartos/frontend/server.js
WorkingDirectory=/cartos/frontend
Restart=always
User=$USER
#Group=sudo

[Install]
WantedBy=multi-user.target"

echo "Installing the backend service."

# Create the systemd service file
echo "$BACKEND_SERVICE_CONTENTS" | sudo tee $BACKEND_SERVICE_FILE > /dev/null

# Reload systemd configuration to recognize the new service
sudo systemctl daemon-reload

# Enable the service to start on boot
sudo systemctl enable cartos-backend.service

# Start the service
sudo systemctl start cartos-backend.service

# Check the status of the service
sudo systemctl status cartos-backend.service

echo "Installing the frontend service."

# Create the systemd service file
echo "$FRONTEND_SERVICE_CONTENTS" | sudo tee $FRONTEND_SERVICE_FILE > /dev/null

# Reload systemd configuration to recognize the new service
sudo systemctl daemon-reload

# Enable the service to start on boot
sudo systemctl enable cartos-frontend.service

# Start the service
sudo systemctl start cartos-frontend.service

# Check the status of the service
sudo systemctl status cartos-frontend.service
