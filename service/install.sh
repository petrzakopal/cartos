#!/bin/bash

# Define service file path and the contents of the service file
SERVICE_FILE="/etc/systemd/system/cartos-backend.service"
SERVICE_CONTENTS="[Unit]
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

# Create the systemd service file
echo "$SERVICE_CONTENTS" | sudo tee $SERVICE_FILE > /dev/null

# Reload systemd configuration to recognize the new service
sudo systemctl daemon-reload

# Enable the service to start on boot
sudo systemctl enable cartos-backend.service

# Start the service
sudo systemctl start cartos-backend.service

# Check the status of the service
sudo systemctl status cartos-backend.service
