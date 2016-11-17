#!/bin/bash
# Dieses Script beinhaltet die einzelnen Schritte die in der Regel nötig sind
# um die Software auf den aktuellsten Stand zu bringen.
# Es ist ausdrücklich nicht Aufgabe dieses Scriptes die Git Versionskontrolle
# zu steuern! Dies ist Aufgabe des Entwicklers. Da heißt vor Aufruf des Scriptes
# muss der Softwarezweig mit z.B. `git checkout` und `git pull` auf den
# gewünschten Stand gebracht werden.

# Exit on error or variable unset
set -o errexit -o nounset

# Stop laufende Instanz
systemctl stop kalibrator.service
# bereinige cargo
#cargo clean
# Bilde neues Release
cargo build --release
# Kopiere neu erstellte Binaries und Assets in das Dateisystem
cp -v ./target/release/kalibrator /usr/bin/kalibrator
# Bibliotheken installieren
cp -rv ./target/release/build/libmodbus-sys-*/out/lib/* /usr/lib/

# Starte Instanz wieder
systemctl start kalibrator.service
