# Qruick

![Showcase of Qruick](./img/showcase.png)

A simple QR-code generator app made with Dioxus

This is made with mobile-first in mind. To try it on an ios emulator, run `dx
serve --platfrom ios`.

## Physical iPhone

Actually getting it signed for a physical iPhone is a bit harder, and requires
a valid signature and `mobileprovision` file linked to your iPhone and developer
signature. Move your mobileprovision file into the root of the build directory
then run `python3 bundle.py -i <signature-id>` to sign the app. You might have
to run it twice. If done correctly, you should now have a valid signed app you
can deploy to your iphone using `ios-deploy`

## Android

I have not tried android yet, but will look at it soon.

## Desktop

Works flawlessly with `dx serve --platform desktop`
