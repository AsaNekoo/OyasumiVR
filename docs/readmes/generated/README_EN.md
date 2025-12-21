linux port of https://github.com/Raphiiko/OyasumiVR maybe (probablly not) one day i will upstream it



very much work in progress (i'm yet to met someone who would care about the port so some features are missing)


if you find a bug in overlay or other linux specific stuff open a bug report in this repo

todo: https://github.com/sofoxe1/OyasumiVR/issues/1

things that should work (open a bug if they don't):

* software brigthness
* sleep detection (both sleep detection and head shake are not linux specific but may behave differently due to agressive optimizations)
* head shake detection
* canceling sleep with a button press (currently all button+trigger+thumbstick)
* slightly buggy overlay
* changing controller bindings
* overlay notifications
* system/gpu power profiles
* frame limiting using mangohud (you need to launch an app with it and oyasumi should do the rest)
* all vrchat automations
* sounds playback (no linux specific code)
* audio device automations

things that should work but are not tested:
* discord rich presence
* system shutdown
* pulsoid integration (no linux specific code)
* sunset automation (no linux specific code)

things that may work:
* osc for other targets then vrchat
