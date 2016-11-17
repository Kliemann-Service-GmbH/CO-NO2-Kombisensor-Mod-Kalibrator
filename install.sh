#!/bin/bash
# Dieses Script beinhaltet die einzelnen Schritte die in der Regel nötig sind
# um die Software auf den aktuellsten Stand zu bringen.
# Es ist ausdrücklich nicht Aufgabe dieses Scriptes die Git Versionskontrolle
# zu steuern! Dies ist Aufgabe des Entwicklers. Da heißt vor Aufruf des Scriptes
# muss der Softwarezweig mit z.B. `git checkout` und `git pull` auf den
# gewünschten Stand gebracht werden.

# Exit on error or variable unset
set -o errexit -o nounset

# Programmdateien installieren
cp -v ./target/release/kalibrator /usr/bin/kalibrator
# Bibliotheken installieren
cp -rv ./target/release/build/libmodbus-sys-*/out/lib/* /usr/lib/

# Systemd Unit File anlegen
cat <<EOF | tee /etc/systemd/system/kalibrator.service
#
# Kalibrator systemd unit file
#
[Unit]
Description="Kalibrator Software für die CO/NO2 Kombisensoren mit Modbus Interface"
After=weston.service
#After=dev-input-input3.device # Scheinbar doch nicht nötig

[Service]
Environment="XDG_RUNTIME_DIR=/run/user/root"
Environment="GDK_BACKEND=wayland"
Environment="XMZ_HARDWARE=0.1.0"
Environment="LANG=de_DE.UTF-8"
ExecStart=/usr/bin/kalibrator &
Restart=always
RestartSec=10

[Install]
WantedBy=graphical.target
EOF

# Unit aktivieren ...
# systemctl daemon-reload # wenn das Unit File geändert wurde
systemctl enable kalibrator.service

# Unit starten
systemctl restart kalibrator.service
