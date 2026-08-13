import type { addOp, div, mult, sub } from "../math/arith.ts";
import type { comp, max, min, ucomp } from "../math/comp.ts";
import type { bin2nb, nb2bin, nb2str, snb2str, str2nb } from "../math/format.ts";
import type { and, or, countBits, clz, bitClear } from "../math/logic.ts";
import type { funnelShift, rol, sar } from "../math/shift.ts";

type a = or<and<str2nb<'f'>, str2nb<'5'>>, str2nb<'fff'>>
type b = addOp<str2nb<'ffffffffffffffff'>, str2nb<'55355'>>;
type c = sub<str2nb<'1'>, str2nb<'10'>>
type d = comp<str2nb<'aaaaaaaaaaaaaaaa'>, str2nb<'ffffffffffffffa'>>
type e = nb2str<funnelShift<str2nb<'0123456789abcdef'>, str2nb<'f0f0f0f0f0f0f0f0'>, [0, 0]>>
type f = nb2str<rol<str2nb<'fffffffff'>, [4,3]>>
type g = nb2str<sar<str2nb<'8000000010000003'>, [5,0]>>
type h = countBits<str2nb<'0123000ffaa'>>
type i = clz<str2nb<'3fff5'>>
type j = mult<str2nb<'a'>, str2nb<'a'>>
type k = nb2bin<bin2nb<'1110111011100110100110100011'>>
type l = div<str2nb<'32341'>, str2nb<'10'>>
type m = snb2str<min<str2nb<'-123'>, str2nb<'-321'>>>
type n = ucomp<str2nb<'ffffffffffffffff'>, str2nb<'321'>>