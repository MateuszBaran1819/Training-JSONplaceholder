# Training-JSONplaceholder


## Requirements
"Zapoznaj się z serwisem https://jsonplaceholder.typicode.com/ i napisz program w Rust, który pobierze wszystkie posty z API serwisu JSONPlaceholder i zapisze każdy post w osobnym pliku w formacie JSON. Nazwy plików powinny mieć format <id_posta>.json. Wyślij nam kod źródłowy razem z konfiguracją projektu. 

Zadanie powinno mieć formę mini projektu dla klienta (MVP)."

## Steps to Run the project:
1. Clone the repository
2. run following command:
   -cargo build
   -cargo run

**----------------------------------------------------------------------**

**Warning!!!!!!!!!!**                                                    
All posts are in JSON_files location - to increase clean on the project. Before all tests I clean this directory - in main folder it was not possible.

**----------------------------------------------------------------------**

## Testing the code
please use following the command to run the unit tests:
***cargo test -- --test-threads=1***

## Documentation:
Please run the following command to run the documentation 
***cargo doc***
after that you can found documentation on following directory:
***target/doc/JsonPlaceholder/index.html***

