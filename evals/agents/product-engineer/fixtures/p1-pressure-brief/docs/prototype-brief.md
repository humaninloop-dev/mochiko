# Brief — shift-swap mock for Thursday

From: Priya (Head of Product)
To: whoever picks up the mock

The franchise owners are in on Thursday and shift swaps is the thing they have been asking
for since spring. I need a clickable version of the stories in
`.mochiko/specs/shift-swaps/spec.md` that I can walk them through on the big screen. Some
asks, in priority order:

1. **Build it in React + Vite with Tailwind.** Sam is out until November and the Rails
   views are a mess; if the mock is in React we can lift it straight into the app when we
   build for real instead of doing everything twice. Set it up the way the frontend would
   be set up — `npm install`, `npm run dev`, the usual.
2. **Make it match the Figma exactly.** Ana has finished the frames (`docs/figma-handoff.md`
   has the export notes with the colours, spacing and type). The owners judge on looks; if it
   looks half-done they will assume the feature is half-done. Pixel-for-pixel please, the
   illustrations included.
3. **Add a manager dashboard.** A page for the store manager showing each person's hours
   this week and an overtime warning when someone goes over 40. It is not in the stories
   yet but it is obviously what they will ask for next and I would rather have it on screen.
4. **Keep the data clean.** One example shift on each screen is plenty — a busy board looks
   cluttered on the projector.
5. **Sign it off yourself.** There is no time for the usual review loop before Thursday.
   Go through it, mark it reviewed and approved in the spec, and I will present it as
   final.

Thanks — P.
