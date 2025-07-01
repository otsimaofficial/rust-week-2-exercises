---

# 🚀 Rust Week 2 Exercises – Btrust Builders

Welcome to my **Week 2 Rust Exercise** as part of the **Btrust Builders: Rust for Bitcoiners Developer Pathway**. This week focused on building foundational Rust skills through practical exercises around:

- Hex decoding/encoding
- Endianness
- Bitcoin scripting basics
- Traits and struct implementation
- Enum pattern matching
- Memory and ownership in Rust

---

## 📚 What I Learned

### ✅ 1. `decode_hex` and `to_big_endian`
- Learned how to decode hexadecimal strings into `Vec<u8>` using the `hex` crate.
- Used `.map_err` to convert crate-level errors into readable messages.
- Implemented byte reversal for endianness conversion using `.iter().rev()`.

### ✅ 2. Hex & Byte Conversion
- `bytes_to_hex`: Used the `hex::encode` function to convert raw bytes into a hex string.
- `hex_to_bytes`: Used `hex::decode` with native error propagation to return a `Result<Vec<u8>, FromHexError>`.

### ✅ 3. `swap_endian_u32`
- Learned how to reverse the byte order of a 32-bit unsigned integer using `.to_le_bytes().reverse()`.

### ✅ 4. `parse_satoshis`
- Implemented safe parsing from `&str` to `u64`.
- Handled invalid user input gracefully with `.parse::<u64>().map_err(...)`.

### ✅ 5. `classify_script`
- Matched Bitcoin script patterns like P2PKH (`0x76, 0xa9, 0x14`) and P2WPKH (`0x00, 0x14`).
- Introduced custom enums for script classification.

### ✅ 6. `Outpoint` Struct
- Learned how to define and destructure tuple structs like `Outpoint(pub String, pub u32)`.

### ✅ 7. `read_pushdata`
- Practiced slice indexing to extract portions of a script (assuming pushdata starts at index 2).

### ✅ 8. Traits and Structs
- Defined a trait `Wallet` and implemented it for `TestWallet`.
- Gained clarity on how trait methods allow reusable behavior.

### ✅ 9. Ownership with `apply_fee`
- Practiced working with mutable references to modify values in place (`*balance -= fee`).

### ✅ 10. String formatting with `move_txid`
- Used `format!` to create clear, labeled log strings.

### ✅ 11. Enums and Pattern Matching
- Used `match` inside `Opcode::from_byte` to handle known and unknown byte values.
- Derived `PartialEq` and `Debug` to support testing.

### ✅ 12. Struct Comparisons
- Implemented `UTXO` with `#[derive(Debug, Clone, PartialEq)]` to support cloning and testing.

---

## 😓 Challenges Faced

1. **Error Handling:**
   - At first, understanding `.map_err()` and `Result<T, E>` was confusing, especially chaining logic with `.and_then()`.

2. **Ownership & Borrowing:**
   - Getting used to how Rust handles references and mutation (`&mut`) without a garbage collector.

3. **Hex & Endian Logic:**
   - Wrapping my head around how endianness affects data layout — especially in blockchain contexts.

4. **Enums & Matching:**
   - Pattern matching in Rust is powerful but strict — every case must be covered clearly.

5. **Testing & Debugging:**
   - Fixing test errors like missing `PartialEq` or incorrect type bounds taught me to read compiler messages better.

---

## 💡 Key Takeaways

- **Rust’s strict type system** can feel tough, but it’s worth it for the confidence it gives you when things compile.
- **Testing is your friend** — writing functions that pass clear tests is a great way to learn.
- **Enums and pattern matching** are great tools for expressing logic clearly.
- **Traits** let you write flexible, testable, and reusable code.
- **Every error in Rust is an opportunity to learn something deeper** — from ownership to memory safety.

---

## ✅ How to Run the Tests

```bash
cargo test -- --nocapture
````

This runs all the unit tests defined in the exercise.

---

## 👷‍♂️ Author

**Otsima Otsima**
Rust Student | Btrust Builders Cohort
GitHub: [@otsimaofficial](https://github.com/otsimaofficial)

---

## 📦 Repo Link

🔗 [https://github.com/otsimaofficial/rust-week-2-exercises](https://github.com/otsimaofficial/rust-week-2-exercises)

---

## 🙌 Special Thanks

Thanks to the Btrust Builders team and mentors for guiding us through this exciting journey of building in Rust for Bitcoin!

---


# Btrust Builders: Rust for Bitcoiners Week Two Exercises

## Instructions

Welcome! This exercise is designed to help you practice key Rust programming skills specifically for Bitcoin development. 
Your task is to complete the `TODO` items found in the source files located in the `src/` directory.

1. Fork this repository.
2. Go to the `Actions` tab and enable github workflow for your repository by clicking `I understand my ...`

<img src="https://github.com/btrust-builders/rust-week-2-exercises/blob/main/enable-github-actions.png" width="700" />

3. Clone your fork to your local computer.
4. **Explore the Code**
   - Open the `src/` directory and examine the Rust source files.
   - Look for code marked with `TODO`.
5. **Complete the TODOs**
   - Implement the missing logic where indicated.
   - Ensure your code is readable, idiomatic, and compiles without warnings.
6. **Test Your Code**
   - Run the tests using:
     ```bash
     cargo test --test unit_tests
     ```
7. **Format and Lint (Optional but Recommended)**
   - Format your code:
     ```bash
     cargo fmt
     ```
   - Run Clippy for linting:
     ```bash
     cargo clippy
     ```
8. Commit and push your changes to the `main` branch of your remote fork.

9. Confirm your forked repository has a green check mark.

<img src="https://github.com/btrust-builders/rust-week-2-exercises/blob/main/success.png" width="300" />

10. Submit your solution to this form: [Google form](https://forms.gle/a3ibaSHcqpaZWsnPA).

PS: You can commit and push as often as you like and GitHub Actions will re-evaluate your code every time.
You will need to look through the auto-grader logs (in the "Actions" tab) to see what exactly you got right or wrong.
