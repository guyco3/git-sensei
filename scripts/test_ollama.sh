#!/bin/bash

MODEL="llama3.2:1b"
ENDPOINT="http://localhost:11434/api/generate"

echo "🧪 Testing Ollama model: $MODEL"
echo "--------------------------------"

# Measure the time it takes for the request to complete
START_TIME=$(date +%s%N)

RESPONSE=$(curl -s -w "\nTIME_TOTAL:%{time_total}\n" -X POST $ENDPOINT -d "{
  \"model\": \"$MODEL\",
  \"prompt\": \"Write a one-word test message.\",
  \"stream\": false
}")

END_TIME=$(date +%s%N)

# Extract and format results
CLEAN_RESPONSE=$(echo "$RESPONSE" | grep -v "TIME_TOTAL")
TIME_TOTAL=$(echo "$RESPONSE" | grep "TIME_TOTAL" | cut -d: -f2)

echo "RAW RESPONSE:"
echo "$CLEAN_RESPONSE" | jq . 2>/dev/null || echo "$CLEAN_RESPONSE"
echo "--------------------------------"
echo "🕒 Total Request Time: ${TIME_TOTAL}s"

# Check if the response is actually empty
TEXT_CHECK=$(echo "$CLEAN_RESPONSE" | jq -r '.response')

if [ "$TEXT_CHECK" == "" ] || [ "$TEXT_CHECK" == "null" ]; then
    echo "❌ ERROR: Ollama returned an empty response string."
    echo "Tip: Try running 'ollama run $MODEL' manually to see if the model is corrupted."
else
    echo "✅ SUCCESS: Ollama is responding correctly."
fi