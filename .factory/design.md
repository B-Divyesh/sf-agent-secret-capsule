# Visual thesis — brutalist concrete and moss

Agent Secret Capsule should feel like a physical containment tool, not a cloud
dashboard. The interface uses the blunt mass of cast concrete for boundaries and
the quiet persistence of moss for the one safe path through them. Hard square
edges, stamped labels, exposed registration marks, and a single organic image
make the security model legible: a credential enters a narrow channel, a command
runs, and only a receipt leaves.

## Palette

The site is intentionally single-mode, painted dark like a sealed service
basement. `ink #F1F0E8` sits on `basalt #171A17` (15.2:1); `dust #B8BDB1` is the
muted text (9.1:1); `slab #252A25` and `rebar #3A413A` form surfaces and rules.
`moss #B8D957` is the action/safe-state color, always paired with near-black
`#11140D` (10.8:1). `rust #F0A36B` warns, `lichen #A9D6B4` confirms, and `clay
#FF8C7A` marks errors. Color never carries state without a word or symbol.

## Type and spacing

Headings use self-hosted DejaVu Serif Bold: weight and chiseled serifs evoke
lettering pressed into wet concrete. UI, terminal, and body copy use self-hosted
DejaVu Sans Mono Regular/Bold so commands and claims share one inspectable
voice. Body text is at least 16px with 1.58 leading and a 68-character measure.
The scale is 16 / 20 / 25 / 40 / clamp(52, 8vw, 108) px. Spacing follows an
8px rhythm with 4px optical corrections; page gutters are 20px on phones,
40px on tablets, and 64px on wide screens.

## Interaction grammar

Controls are rectangular 48px minimum targets with a 2px border and a 4px
pressed translation, like a mechanical switch. Focus is a 3px moss outline
with 3px clearance. Independent proof points appear as bolted slabs; narrative
content is grouped by whitespace rather than card grids. On phones, navigation
drops secondary anchors and the containment diagram stacks into a vertical
sequence. Copy buttons change their visible label and announce the result.

## Motion policy

Only state changes move: the hero receipt settles upward 12px over 240ms, the
demo secret is visibly replaced over 180ms, and buttons depress over 120ms.
Nothing loops. Under `prefers-reduced-motion: reduce`, transforms and smooth
scrolling are removed and state changes are instant while remaining explicit.

## Asset plan and provenance

The hero uses one original generated editorial texture: a top-down monolithic
concrete capsule split by a narrow channel of living moss, with no text, logos,
people, screens, gradients, or security clichés. It is generated specifically
for this product with `/opt/fleet/lib/gen-image.sh` using the factory image
deployment, then cropped and compressed to WebP at no more than 300 KB. The
source prompt and deployment sidecar are retained beside the asset. The small
capsule mark, arrows, and receipt diagram are hand-authored CSS/HTML geometric
forms; they contain no third-party artwork.

For polish 1, `capsule-social.webp` (1200×630) and `apple-touch-icon.png`
(180×180) were composed by cropping the existing original capsule artwork with
ImageMagick. They introduce no new artwork or text and preserve the concrete-
and-moss visual language.
