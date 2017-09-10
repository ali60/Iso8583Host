# Iso8583Host
A simple Iso8583 server in Rust
# Building
To build the application, simple use cargo

cargo build

cargo run

"127.0.0.1"
"2020"
start listening on 127.0.0.1:2020

# message customization
All iso8583 message formats is defined in iso8583_message_format.xml

<?xml version="1.0" encoding="UTF-8"?>
<iso_transactions version="87">
<transaction mti="0200">
    <field num="1" format="BINARY" length="8" value="" />
    <field num="2" format="LLVAR" length="19" value="" />
    <field num="3" format="NUMERIC" length="6"  value="" />
    <field num="4" format="AMOUNT" length="12" value="" />


