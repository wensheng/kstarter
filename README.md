# kstarter

`kstarter` creates a detached Kit desktop session and exits. It starts `Xvfb`
and `jwm`, writes session metadata, and leaves the desktop available for tools
such as `kitwin`, `kommander`, `kutter`, `kviewer`, and `kstreamer`.

```bash
kstarter --session demo --width 1920 --height 1200 --exec xterm
```

The session can later be attached, controlled, viewed, captured, streamed, or
terminated by the other Kit tools.

## Requirements

At runtime, `kstarter` expects the external tools used by Kit desktop sessions,
including `Xvfb` and `jwm`.
