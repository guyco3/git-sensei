import time
import subprocess

def measure_latency():
    start_time = time.time()
    try:
        result = subprocess.run(
            ["./target/debug/gitsensei", "suggest"],
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            check=True
        )
        end_time = time.time()
        latency = (end_time - start_time) * 1000  # Convert to milliseconds
        print(f"Latency: {latency:.2f} ms")
        print("Output:", result.stdout)
    except subprocess.CalledProcessError as e:
        print("Error:", e.stderr)

if __name__ == "__main__":
    measure_latency()