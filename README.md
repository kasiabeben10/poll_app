# Poll App - Zdecentralizowana aplikacja głosowania na Solana

Aplikacja Poll App to zdecentralizowana platforma do tworzenia i uczestnictwa w ankietach, zbudowana na blockchainie Solana przy użyciu frameworka Anchor. Zapewnia przejrzysty i bezpieczny system głosowania z funkcjami zapobiegającymi wielokrotnemu głosowaniu.

## 🚀 Funkcjonalności

- **Inicjalizacja użytkownika**: Utwórz profil użytkownika do zarządzania ankietami
- **Tworzenie ankiet**: Twórz ankiety z maksymalnie 5 opcjami i opcjonalnym czasem trwania
- **Bezpieczne głosowanie**: System zapobiegający wielokrotnemu głosowaniu przy użyciu kryptograficznych commitmentów
- **Wyświetlanie wyników**: Przeglądaj wyniki ankiet w czasie rzeczywistym
- **Znajdowanie zwycięzców**: Automatyczne określanie opcji z największą liczbą głosów

## 📋 Wymagania

- Rust (najnowsza stabilna wersja)
- Solana CLI tools
- Anchor Framework
- Node.js (do narzędzi deweloperskich)

## 🔧 Instalacja

1. **Klonowanie repozytorium**
```bash
git clone <repository-url>
cd poll-app
```

2. **Instalacja zależności Rust**
```bash
cargo build
```

3. **Konfiguracja Solany dla sieci lokalnej**
```bash
solana config set --url localhost
solana-keygen new --outfile ~/.config/solana/id.json
```

4. **Uruchomienie lokalnego validatora Solany**
```bash
solana-test-validator
```

5. **Zbudowanie i wdrożenie programu**
```bash
anchor build
anchor deploy
```

## 🎯 Użytkowanie

### Inicjalizacja użytkownika

Przed utworzeniem pierwszej ankiety musisz zainicjalizować swój profil użytkownika:

```bash
cargo run -- initialize-user
```

### Tworzenie ankiety

Utwórz nową ankietę z pytaniem, opcjami i opcjonalnym czasem trwania:

```bash
# Ankieta bez limitu czasu
cargo run -- create-poll -q "Jaki jest Twój ulubiony kolor?" -o "Czerwony" -o "Niebieski" -o "Zielony" -d 0

# Ankieta z limitem czasu (3600 sekund = 1 godzina)
cargo run -- create-poll -q "Gdzie powinniśmy się spotkać?" -o "Park" -o "Kawiarnia" -o "Centrum handlowe" -d 3600
```

**Parametry:**
- `-q, --question`: Pytanie ankiety (maksymalnie 256 znaków)
- `-o, --options`: Opcje odpowiedzi (minimum 2, maksimum 5)
- `-d, --duration`: Czas trwania w sekundach (0 = bez limitu)

### Głosowanie

Oddaj głos w ankiecie podając indeks opcji i adres ankiety:

```bash
cargo run -- vote -o 1 -p DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3
```

**Parametry:**
- `-o, --option-index`: Numer opcji (zaczynając od 1)
- `-p, --poll-address`: Adres ankiety na blockchain

### Wyświetlanie wyników

Zobacz aktualne wyniki ankiety:

```bash
cargo run -- view-poll -p DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3
```

### Znajdowanie zwycięzcy

Wyświetl opcję(e) z największą liczbą głosów:

```bash
cargo run -- get-winner -p DvUMKKX58dNUySNKZo7buMZcniuYM1pSxWZxtqAGLne3
```

## 🔐 Bezpieczeństwo

### System zapobiegający wielokrotnemu głosowaniu

Aplikacja używa zaawansowanego systemu kryptograficznego, aby zapobiec wielokrotnemu głosowaniu:

1. **Generowanie commitment**: Dla każdego głosu tworzony jest unikalny commitment na podstawie klucza publicznego użytkownika i seed ankiety
2. **Weryfikacja**: System sprawdza, czy commitment już istnieje przed zaakceptowaniem głosu
3. **Anonimowość**: Commitment nie ujawnia tożsamości głosującego ani wybranej opcji

### Walidacja danych

- Pytania są ograniczone do 256 znaków
- Wymagane są minimum 2 i maksimum 5 opcji
- Wszystkie opcje muszą być niepuste
- Czas trwania musi być nieujemny

## 📊 Struktura danych

### Poll (Ankieta)
```rust
pub struct Poll {
    pub question: String,        // Pytanie ankiety
    pub options: Vec<String>,    // Lista opcji
    pub votes: Vec<u32>,         // Liczba głosów na każdą opcję
    pub voters: Vec<[u8; 32]>,   // Lista commitmentów głosujących
    pub bump: u8,                // Bump seed dla PDA
    pub created_at: i64,         // Timestamp utworzenia
    pub duration: i64,           // Czas trwania w sekundach
    pub voter_count: u32,        // Całkowita liczba głosów
    pub seed: [u8; 8],          // Seed dla generowania commitmentów
}
```

### UserStats (Statystyki użytkownika)
```rust
pub struct UserStats {
    pub user: Pubkey,           // Klucz publiczny użytkownika
    pub polls_count: u32,       // Liczba utworzonych ankiet
    pub bump: u8,               // Bump seed dla PDA
}
```

## 🛠️ Architektura

Aplikacja składa się z dwóch głównych komponentów:

1. **Program Anchor** (`lib.rs`): Smart contract na blockchain Solana
2. **Klient CLI** (`main.rs` + moduły): Interfejs wiersza poleceń do interakcji z programem

### Program Anchor

Program zawiera następujące instrukcje:
- `initialize_user`: Inicjalizacja profilu użytkownika
- `create_poll`: Tworzenie nowej ankiety
- `vote`: Oddawanie głosu
- `get_results`: Pobieranie wyników ankiety

### Klient CLI

Klient CLI jest podzielony na moduły:
- `create_poll.rs`: Obsługa tworzenia ankiet
- `vote.rs`: Obsługa głosowania
- `view_poll.rs`: Wyświetlanie wyników i zwycięzców
- `initialize_user.rs`: Inicjalizacja użytkownika

## 🔍 Przykład użycia

```bash
# 1. Inicjalizacja użytkownika
cargo run -- initialize-user

# 2. Utworzenie ankiety
cargo run -- create-poll -q "Jaka masz ocenę z Techniki Cyfrowej?" -o "3.0" -o "3.5" -o "4.0" -o "4.5" -o "5.0" -d 1800

# Wyjście:
# ✅ Poll created successfully!
# 🔗 Poll address: ABC123...
# 📝 Question: Jaka masz ocenę z Techniki Cyfrowej?
# 📌 Options:
#   0. 2.0
#   1. 3.0  
#   2. 3.5
#   3. 4.0
#   4. 4.5
# Validity: 30 min

# 3. Głosowanie
cargo run -- vote -o 2 -p ABC123...

# Wyjście:
# 🗳️ Your vote has been submitted successfully!
# ✅ You voted for option 3 in poll ABC123...

# 4. Sprawdzenie wyników
cargo run -- view-poll -p ABC123...

# Wyjście:
# 📊 Poll Results 📊
# 🔹 Question: Jaka masz ocenę z Techniki Cyfrowej?
# 
# Options:
# 0: 2.0 - 1 votes
# 1: 3.0 - 3 votes
# 2: 3.5 - 0 votes
# 3: 4.0 - 0 votes
# 4: 4.5 - 0 votes
# -----------
# Total voters: 4

# 5. Znajdowanie zwycięzcy
cargo run -- get-winner -p ABC123...

# Wyjście:
# 🏆 Winner(s) 🏆
# Total votes: 4
# Winning options (3 votes each):
#   ✨ KFC
```

## 🐛 Rozwiązywanie problemów

### Błąd "User not initialized"
```bash
cargo run -- initialize-user
```

### Błąd "Poll is closed"
Ankieta przekroczyła swój limit czasu. Sprawdź czas utworzenia i czas trwania ankiety.

### Błąd "User has already voted"
Każdy użytkownik może głosować tylko raz w danej ankiecie.

### Błąd "Invalid option index"
Sprawdź, czy podany indeks opcji jest prawidłowy (liczba od 1 do liczby opcji).