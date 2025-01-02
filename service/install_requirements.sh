#!/bin/bash

# Update package list
sudo apt update

# Install pcscd, pcsc-tools, and libpcsclite1
sudo apt install -y pcscd pcsc-tools libpcsclite1

# Install libnfc6, libnfc-dev, and libnfc-bin
sudo apt-get install -y libnfc6 libnfc-dev libnfc-bin

# Optional: Notify user that installation is complete
#echo "Installation complete!"

LIBNFC_FILE="/etc/nfc/libnfc.conf"
LIBNFC_CONF_CONTENTS="
# Allow device auto-detection (default: true)
# Note: if this is set to false, a device must be configured using a device-configuration entry
allow_autoscan = true

# Allow intrusive auto-detection (default: false)
allow_intrusive_scan = false

# Set log level (default: error)
# Valid log levels are (in order of verbosity): 0 (none), 1 (error), 2 (info), 3 (debug)
log_level = 1

# Manually configure your NFC device
device.name = "ACR122U"
device.connstring = "pn532_uart:/dev/ttyUSB0"
"
NFC_99_FILE="/etc/udev/rules.d/99-nfc.rules"

NFC_99_CONTENTS="
# ACR122U
SUBSYSTEMS=="usb", ATTRS{idVendor}=="072f", ATTRS{idProduct}=="2200", MODE="0660", GROUP="plugdev"

# PN533
SUBSYSTEMS=="usb", ATTRS{idVendor}=="054c", ATTRS{idProduct}=="0193", MODE="0660", GROUP="plugdev"

# SCL3711
SUBSYSTEMS=="usb", ATTRS{idVendor}=="04e6", ATTRS{idProduct}=="5591", MODE="0660", GROUP="plugdev"
"

echo "$LIBNFC_CONF_CONTENTS" | sudo tee $LIBNFC_FILE > /dev/null

echo "$NFC_99_CONTENTS" | sudo tee $NFC_99_FILE > /dev/null

sudo usermod -a -G plugdev $USER

sudo systemctl enable pcscd
sudo systemctl start pcscd

sudo udevadm control --reload-rules
sudo udevadm trigger
