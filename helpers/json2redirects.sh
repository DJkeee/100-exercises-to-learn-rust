#!/bin/bash

# Проверяем, что JSON-файл передан в качестве аргумента
if [ "$#" -ne 1 ]; then
  echo "Usage: $0 <input_json_file>"
  exit 1
fi

input_file=$1

# Используем jq для парсинга JSON и форматирования вывода
jq -r 'to_entries[] | "/" + .value + " " + .key' "$input_file"
