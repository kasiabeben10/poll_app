# Poll CLI - Solana Poll Application Command Line Interface

Kompletny CLI dla aplikacji ankietowej na Solana blockchain.

## Instalacja

1. Upewnij się, że masz zainstalowany Rust:
```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

2. Sklonuj i zbuduj projekt:
```bash
git clone <your-repo>
cd poll-cli
cargo build --release
```

3. (Opcjonalnie) Zainstaluj globalnie:
```bash
cargo install --path .
```

## Konfiguracja

CLI automatycznie używa klucza z `~/.config/solana/id.json`. Możesz także określić własny plik klucza:

```bash
poll-cli --keypair-path /path/to/your/keypair.json <command>
```

## Użycie

### Tworzenie ankiety

```bash
# Podstawowa ankieta
poll-cli create-poll "Jaki jest Twój ulubiony kolor?" --options "Czerwony,Niebieski,Zielony"

# Ankieta z limitem czasu (3600 sekund = 1 godzina)
poll-cli create-poll "Gdzie idziemy na lunch?" --options "Pizza,Sushi,Burger" --duration 3600

# Ankieta bez limitu czasu
poll-cli create-poll "Czy podoba Ci się ten projekt?" --options "Tak,Nie" --duration 0
```

### Głosowanie

```bash
# Głosuj na opcję 0 (pierwszą opcję)
poll-cli vote <POLL_ADDRESS> 0

# Głosuj na opcję 1 (drugą opcję)
poll-cli vote <POLL_ADDRESS> 1
```

### Sprawdzanie wyników

```bash
# Pokaż szczegółowe wyniki z wykresami
poll-cli results <POLL_ADDRESS>

# Pokaż tylko zwycięzcę
poll-cli winner <POLL_ADDRESS>
```

### Zarządzanie ankietami

```bash
# Lista Twoich ankiet
poll-cli list-polls

# Szczegółowe informacje o ankiecie
poll-cli info <POLL_ADDRESS>
```

### Opcje sieci

```bash
# Użyj devnet (domyślnie)
poll-cli --cluster devnet create-poll "Test?" --options "A,B"

# Użyj mainnet
poll-cli --cluster mainnet results <POLL_ADDRESS>

# Użyj testnet
poll-cli --cluster testnet vote <POLL_ADDRESS> 0

# Użyj niestandardowego RPC
poll-cli --cluster "https://api.mainnet-beta.solana.com" info <POLL_ADDRESS>
```

## Przykłady użycia

### 1. Tworzenie i udział w ankiecie

```bash
# 1. Utwórz ankietę
poll-cli create-poll "Która technologia jest najlepsza?" --options "Rust,Python,JavaScript,Go"

# Zapisz adres ankiety z output'u
export POLL_ADDR="<POLL_ADDRESS_FROM_OUTPUT>"

# 2. Zagłosuj
poll-cli vote $POLL_ADDR 0  # Głos na Rust

# 3. Sprawdź wyniki
poll-cli results $POLL_ADDR

# 4. Zobacz zwycięzcę
poll-cli winner $POLL_ADDR
```

### 2. Ankieta z limitem czasu

```bash
# Utwórz ankietę na 1 godzinę
poll-cli create-poll "Co robimy dziś wieczorem?" \
    --options "Kino,Restauracja,Zostajemy w domu" \
    --duration 3600

# Sprawdź status ankiety
poll-cli info $POLL_ADDR
```

### 3. Zarządzanie wieloma ankietami

```bash
# Zobacz wszystkie swoje ankiety
poll-cli list-polls

# Sprawdź szczegóły konkretnej ankiety
poll-cli info <POLL_ADDRESS>
```

## Format wyników

CLI wyświetla wyniki w kolorowym, przyjaznym dla użytkownika formacie:

```
=== POLL RESULTS ===
Question: Jaki jest Twój ulubiony kolor?
Total Votes: 15

Results:
  0: Czerwony (8 votes, 53.3%)
     ████████████████████████████████████████████████████
  1: Niebieski (4 votes, 26.7%)
     ███████████████████████████████████
  2: Zielony (3 votes, 20.0%)
     ████████████████████████████████
```

## Kody błędów

- **InvalidOption**: Nieprawidłowy indeks opcji
- **PollClosed**: Ankieta jest zamknięta (przekroczono limit czasu)
- **AlreadyVoted**: Użytkownik już głosował w tej ankiecie

## Struktura projektu

```
poll-cli/
├── Cargo.toml              # Zależności i konfiguracja
├── src/
│   ├── main.rs             # Główna logika CLI
│   └── poll_program.rs     # Definicje struktur programu
└── README.md
```

## Rozwój

Aby dodać nowe funkcjonalności:

1. Dodaj nowe komendy do enum `Commands` w `main.rs`
2. Zaimplementuj odpowiednie funkcje obsługi
3. Dodaj obsługę w funkcji `main()`