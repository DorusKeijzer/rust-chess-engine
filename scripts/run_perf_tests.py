
import time
import subprocess


def run_perft_test():
    # Replace this with your actual perft test command
    command = './target/release/chess perft'
    result = subprocess.run(command, shell=True,
                            capture_output=True, text=True)
    return result.stdout.strip()


output = run_perft_test()

with open("README.md", "r") as f:
    readme_content = f.read()

# Find the section to update in README
start_marker = "## Perft Results"
end_marker = "<!-- End of Perft Results -->"

new_content = f"{start_marker}\n\n"
new_content += "Last updated: " + time.strftime("%Y-%m-%d %H:%M:%S") + "\n\n"
new_content += output
new_content += f"\n{end_marker}"

updated_readme = readme_content.split(start_marker)[0] + new_content


with open("README.md", "w") as f:
    f.write(updated_readme)

print("README.md updated with new perft results.")
