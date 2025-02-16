# Backup di Emergenza Group 32

## Descrizione

Questo programma, scritto interamente in Rust, consente di eseguire un backup di emergenza dei file da una directory sorgente a una directory di destinazione. Include funzionalità per:

- Copiare file specifici o intere directory.
- Generare un file di log contenente la durata del backup e la dimensione totale dei dati copiati.
- Riprodurre un suono di conferma al termine del backup.
- Integrare una finestra di conferma per avviare o annullare il backup.
- Funzionare come un servizio in background su macOS, Windows e Linux utilizzando la libreria `auto-launch`.

## Installazione

### Prerequisiti

Assicurati di avere installato:

- [Rust](https://www.rust-lang.org/) e Cargo
- Il package manager `homebrew` (per macOS, se necessario)

### Clonazione del Repository

```bash
git clone https://github.com/ProgrammazioneDiSistema2024-IA-ZZ/Group-32
cd progetto_g32
```

### Installazione delle Dipendenze

```bash
cargo build --release
```

oppure in base al sistema operativo eseguire uno dei 3 file (.sh e .bat)

## Utilizzo

### Esecuzione Manuale

Puoi eseguire il programma manualmente con:

```bash
cargo run
```

Oppure, se hai compilato il progetto:

```bash
target/release/progetto_g32
```

### Configurazione del Backup

Il programma al primo avvio, lancerà una finestra di configurazione per definire i path sorgente e destinazione e installare il programma in background.
Se si intende modificare i path di backup, basterà aprire l'eseguibile "setup" in cui sarà possibile modificare le informazioni fornite al primo avvio.

### Avvio Automatico

Per rimuovere il servizio:

```bash
target/release/uninstall
```

## Struttura del Progetto

```
/
|-- audio_backup/
|-- configuration_csv/
|-- src/
|   |-- bin/
|   |-- main.rs                     # Punto di ingresso dell'applicazione
|   |-- main_configuration.rs       # Controlli all'avvio dell'applicazione
|   |-- mouse_input.rs              # Gestione degli input del mouse
|   |-- backup.rs                   # Funzioni per la copia dei file
|   |-- cpu_logger.rs               # Gestione del file di log
|   |-- audio.rs                    # Riproduzione del suono di conferma
|   |-- lib.rs
|   |-- confirmation_window.rs      # Finestra di conferma
|   |-- configuration_window.rs     # Finestra di configurazione/setup
|-- Cargo.toml                      # File di configurazione di Rust
|-- cpu_usage_log.txt               # File contenente le stampe con il logging della CPU
|-- lin_build.sh                    # Per creare in automatico la release per Linux
|-- macos_build.sh                  # Per creare in automatico la release per MacOs
|-- windows_build.bat               # Per creare in automatico la release per Windows
```

## Dipendenze Utilizzate

- `sysinfo` (0.32.0) - Per monitorare l'utilizzo della CPU e altre informazioni di sistema.
- `chrono` (0.4) - Per ottenere timestamp precisi per i log.
- `lazy_static` (1.4) - Per inizializzare variabili globali come le dimensioni dello schermo.
- `scrap` (0.5) - Per catturare screenshot dello schermo.
- `rdev` (0.4) - Per intercettare eventi del mouse e della tastiera.
- `rodio` (0.14.0) - Per riprodurre un suono di conferma alla fine del backup.
- `device_query` (0.2) - Per ottenere informazioni sui dispositivi di input (mouse, tastiera).
- `egui` (0.29.1) - Per la gestione dell'interfaccia grafica.
- `eframe` (0.29.1) - Framework per creare finestre grafiche con `egui`.
- `rfd` (0.12) - Per gestire finestre di dialogo native, come la selezione di file.
- `auto-launch` (0.5.0) - Per avviare automaticamente il programma all'accensione del sistema.

## Contributi

- Matteo Vincenzo Petrera 331356
- Davide Proglio 324103
- Gianluca Maida 334263

## Licenza

MIT License. Sentiti libero di usare e modificare il codice come preferisci.
