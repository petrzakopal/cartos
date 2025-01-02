#!/bin/bash

# Define service file path and the contents of the service file
SERVICE_FILE="/etc/systemd/system/cartos.service"
SERVICE_CONTENTS="[Unit]
Description=Cartos
After=multi-user.target usbutils.target pcscd.service pcscd.socket
Requires=pcscd.service pcscd.socket

[Service]
ExecStart=/cartos/app
WorkingDirectory=/cartos/
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
sudo systemctl enable cartos.service

# Start the service
sudo systemctl start cartos.service

# Check the status of the service
sudo systemctl status cartos.service
