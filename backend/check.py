import json

out = []
has_messages = False

with open("check.json") as f:
    for line in f:
        try:
            obj = json.loads(line)
        except:
            continue

        if obj.get("reason") != "compiler-message":
            continue

        msg = obj.get("message", {})
        level = msg.get("level")

        if level not in ("error", "warning"):
            continue

        spans = msg.get("spans", [])
        if not spans:
            continue

        has_messages = True
        span = spans[0]

        out.append(f"### {level.title()}: {msg.get('message', '')}")
        out.append(f"File: {span.get('file_name')}:{span.get('line_start')}")
        out.append("```rust")
        out.append(msg.get("rendered", "").rstrip())
        out.append("```\n")

if not has_messages:
    with open("check.md", "w") as w:
        w.write("# Cargo Check Report\n\nNo warnings or errors. Build success.\n")
else:
    with open("check.md", "w") as w:
        w.write("# Cargo Check Report\n\n" + "\n".join(out))
