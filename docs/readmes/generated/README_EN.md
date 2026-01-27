linux port of https://github.com/Raphiiko/OyasumiVR


very much work in progress


if you find a bug in overlay or other linux specific stuff open a bug report in this repo

todo: https://github.com/sofoxe1/OyasumiVR/issues/1

things that should work (open a bug if they don't):

* software brigthness
* sleep detection (both sleep detection and head shake are not linux specific but may behave differently due to optimizations)
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
* discord rich presence

things that should work but are not tested:
* system shutdown
* pulsoid integration (no linux specific code)
* sunset automation (no linux specific code)

things that may work:
* osc for other targets then vrchat
