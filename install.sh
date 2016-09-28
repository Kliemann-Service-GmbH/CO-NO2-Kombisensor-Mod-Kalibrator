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

# Systemd Unit File anlegen
cat <<EOF | tee /etc/systemd/system/kalibrator.service
#
# xMZ-Mod-Touch-Server systemd unit file
#
[Unit]
Description="Kalibrator Software für die CO/NO2 Kombisensoren mit Modbus Interface"
After=multi-user.target
[Service]
ExecStart=/usr/bin/kalibrator &
[Install]
WantedBy=multi-user.target
EOF

# Unit aktivieren ...
systemctl enable kalibrator.service

# Unit starten
systemctl restart kalibrator.service
