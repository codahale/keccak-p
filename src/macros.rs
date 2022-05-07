#![macro_use]

macro_rules! copy_from_state {
    (
        $lanes:ident,
        $a_ba: ident, $a_be: ident, $a_bi: ident, $a_bo: ident, $a_bu: ident,
        $a_ga: ident, $a_ge: ident, $a_gi: ident, $a_go: ident, $a_gu: ident,
        $a_ka: ident, $a_ke: ident, $a_ki: ident, $a_ko: ident, $a_ku: ident,
        $a_ma: ident, $a_me: ident, $a_mi: ident, $a_mo: ident, $a_mu: ident,
        $a_sa: ident, $a_se: ident, $a_si: ident, $a_so: ident, $a_su: ident,
        $b_ba: ident, $b_be: ident, $b_bi: ident, $b_bo: ident, $b_bu: ident,
        $b_ga: ident, $b_ge: ident, $b_gi: ident, $b_go: ident, $b_gu: ident,
        $b_ka: ident, $b_ke: ident, $b_ki: ident, $b_ko: ident, $b_ku: ident,
        $b_ma: ident, $b_me: ident, $b_mi: ident, $b_mo: ident, $b_mu: ident,
        $b_sa: ident, $b_se: ident, $b_si: ident, $b_so: ident, $b_su: ident,
        $c_a: ident, $c_e: ident, $c_i: ident, $c_o: ident, $c_u: ident,
        $d_a: ident, $d_e: ident, $d_i: ident, $d_o: ident, $d_u: ident,
        $e_ba: ident, $e_be: ident, $e_bi: ident, $e_bo: ident, $e_bu: ident,
        $e_ga: ident, $e_ge: ident, $e_gi: ident, $e_go: ident, $e_gu: ident,
        $e_ka: ident, $e_ke: ident, $e_ki: ident, $e_ko: ident, $e_ku: ident,
        $e_ma: ident, $e_me: ident, $e_mi: ident, $e_mo: ident, $e_mu: ident,
        $e_sa: ident, $e_se: ident, $e_si: ident, $e_so: ident, $e_su: ident,
    ) => {
        let mut $a_ba = $lanes[0];
        let mut $a_be = $lanes[1];
        let mut $a_bi = $lanes[2];
        let mut $a_bo = $lanes[3];
        let mut $a_bu = $lanes[4];
        let mut $a_ga = $lanes[5];
        let mut $a_ge = $lanes[6];
        let mut $a_gi = $lanes[7];
        let mut $a_go = $lanes[8];
        let mut $a_gu = $lanes[9];
        let mut $a_ka = $lanes[10];
        let mut $a_ke = $lanes[11];
        let mut $a_ki = $lanes[12];
        let mut $a_ko = $lanes[13];
        let mut $a_ku = $lanes[14];
        let mut $a_ma = $lanes[15];
        let mut $a_me = $lanes[16];
        let mut $a_mi = $lanes[17];
        let mut $a_mo = $lanes[18];
        let mut $a_mu = $lanes[19];
        let mut $a_sa = $lanes[20];
        let mut $a_se = $lanes[21];
        let mut $a_si = $lanes[22];
        let mut $a_so = $lanes[23];
        let mut $a_su = $lanes[24];
        let mut $b_ba: u64;
        let mut $b_be: u64;
        let mut $b_bi: u64;
        let mut $b_bo: u64;
        let mut $b_bu: u64;
        let mut $b_ga: u64;
        let mut $b_ge: u64;
        let mut $b_gi: u64;
        let mut $b_go: u64;
        let mut $b_gu: u64;
        let mut $b_ka: u64;
        let mut $b_ke: u64;
        let mut $b_ki: u64;
        let mut $b_ko: u64;
        let mut $b_ku: u64;
        let mut $b_ma: u64;
        let mut $b_me: u64;
        let mut $b_mi: u64;
        let mut $b_mo: u64;
        let mut $b_mu: u64;
        let mut $b_sa: u64;
        let mut $b_se: u64;
        let mut $b_si: u64;
        let mut $b_so: u64;
        let mut $b_su: u64;
        let mut $c_a = $a_ba ^ $a_ga ^ $a_ka ^ $a_ma ^ $a_sa;
        let mut $c_e = $a_be ^ $a_ge ^ $a_ke ^ $a_me ^ $a_se;
        let mut $c_i = $a_bi ^ $a_gi ^ $a_ki ^ $a_mi ^ $a_si;
        let mut $c_o = $a_bo ^ $a_go ^ $a_ko ^ $a_mo ^ $a_so;
        let mut $c_u = $a_bu ^ $a_gu ^ $a_ku ^ $a_mu ^ $a_su;
        let mut $d_a: u64;
        let mut $d_e: u64;
        let mut $d_i: u64;
        let mut $d_o: u64;
        let mut $d_u: u64;
        let mut $e_ba: u64;
        let mut $e_be: u64;
        let mut $e_bi: u64;
        let mut $e_bo: u64;
        let mut $e_bu: u64;
        let mut $e_ga: u64;
        let mut $e_ge: u64;
        let mut $e_gi: u64;
        let mut $e_go: u64;
        let mut $e_gu: u64;
        let mut $e_ka: u64;
        let mut $e_ke: u64;
        let mut $e_ki: u64;
        let mut $e_ko: u64;
        let mut $e_ku: u64;
        let mut $e_ma: u64;
        let mut $e_me: u64;
        let mut $e_mi: u64;
        let mut $e_mo: u64;
        let mut $e_mu: u64;
        let mut $e_sa: u64;
        let mut $e_se: u64;
        let mut $e_si: u64;
        let mut $e_so: u64;
        let mut $e_su: u64;
    };
}

macro_rules! double_round {
    (
        $rc_a: tt, $rc_b: tt,
        $a_ba: ident, $a_be: ident, $a_bi: ident, $a_bo: ident, $a_bu: ident,
        $a_ga: ident, $a_ge: ident, $a_gi: ident, $a_go: ident, $a_gu: ident,
        $a_ka: ident, $a_ke: ident, $a_ki: ident, $a_ko: ident, $a_ku: ident,
        $a_ma: ident, $a_me: ident, $a_mi: ident, $a_mo: ident, $a_mu: ident,
        $a_sa: ident, $a_se: ident, $a_si: ident, $a_so: ident, $a_su: ident,
        $b_ba: ident, $b_be: ident, $b_bi: ident, $b_bo: ident, $b_bu: ident,
        $b_ga: ident, $b_ge: ident, $b_gi: ident, $b_go: ident, $b_gu: ident,
        $b_ka: ident, $b_ke: ident, $b_ki: ident, $b_ko: ident, $b_ku: ident,
        $b_ma: ident, $b_me: ident, $b_mi: ident, $b_mo: ident, $b_mu: ident,
        $b_sa: ident, $b_se: ident, $b_si: ident, $b_so: ident, $b_su: ident,
        $c_a: ident, $c_e: ident, $c_i: ident, $c_o: ident, $c_u: ident,
        $d_a: ident, $d_e: ident, $d_i: ident, $d_o: ident, $d_u: ident,
        $e_ba: ident, $e_be: ident, $e_bi: ident, $e_bo: ident, $e_bu: ident,
        $e_ga: ident, $e_ge: ident, $e_gi: ident, $e_go: ident, $e_gu: ident,
        $e_ka: ident, $e_ke: ident, $e_ki: ident, $e_ko: ident, $e_ku: ident,
        $e_ma: ident, $e_me: ident, $e_mi: ident, $e_mo: ident, $e_mu: ident,
        $e_sa: ident, $e_se: ident, $e_si: ident, $e_so: ident, $e_su: ident,
    ) => {
        $d_a = $c_u ^ $c_e.rotate_left(1);
        $d_e = $c_a ^ $c_i.rotate_left(1);
        $d_i = $c_e ^ $c_o.rotate_left(1);
        $d_o = $c_i ^ $c_u.rotate_left(1);
        $d_u = $c_o ^ $c_a.rotate_left(1);
        $a_ba ^= $d_a;
        $b_ba = $a_ba;
        $a_ge ^= $d_e;
        $b_be = $a_ge.rotate_left(44);
        $a_ki ^= $d_i;
        $b_bi = $a_ki.rotate_left(43);
        $a_mo ^= $d_o;
        $b_bo = $a_mo.rotate_left(21);
        $a_su ^= $d_u;
        $b_bu = $a_su.rotate_left(14);
        $e_ba = $b_ba ^ ((!$b_be) & $b_bi);
        $e_ba ^= $rc_a;
        $c_a = $e_ba;
        $e_be = $b_be ^ ((!$b_bi) & $b_bo);
        $c_e = $e_be;
        $e_bi = $b_bi ^ ((!$b_bo) & $b_bu);
        $c_i = $e_bi;
        $e_bo = $b_bo ^ ((!$b_bu) & $b_ba);
        $c_o = $e_bo;
        $e_bu = $b_bu ^ ((!$b_ba) & $b_be);
        $c_u = $e_bu;
        $a_bo ^= $d_o;
        $b_ga = $a_bo.rotate_left(28);
        $a_gu ^= $d_u;
        $b_ge = $a_gu.rotate_left(20);
        $a_ka ^= $d_a;
        $b_gi = $a_ka.rotate_left(3);
        $a_me ^= $d_e;
        $b_go = $a_me.rotate_left(45);
        $a_si ^= $d_i;
        $b_gu = $a_si.rotate_left(61);
        $e_ga = $b_ga ^ ((!$b_ge) & $b_gi);
        $c_a ^= $e_ga;
        $e_ge = $b_ge ^ ((!$b_gi) & $b_go);
        $c_e ^= $e_ge;
        $e_gi = $b_gi ^ ((!$b_go) & $b_gu);
        $c_i ^= $e_gi;
        $e_go = $b_go ^ ((!$b_gu) & $b_ga);
        $c_o ^= $e_go;
        $e_gu = $b_gu ^ ((!$b_ga) & $b_ge);
        $c_u ^= $e_gu;
        $a_be ^= $d_e;
        $b_ka = $a_be.rotate_left(1);
        $a_gi ^= $d_i;
        $b_ke = $a_gi.rotate_left(6);
        $a_ko ^= $d_o;
        $b_ki = $a_ko.rotate_left(25);
        $a_mu ^= $d_u;
        $b_ko = $a_mu.rotate_left(8);
        $a_sa ^= $d_a;
        $b_ku = $a_sa.rotate_left(18);
        $e_ka = $b_ka ^ ((!$b_ke) & $b_ki);
        $c_a ^= $e_ka;
        $e_ke = $b_ke ^ ((!$b_ki) & $b_ko);
        $c_e ^= $e_ke;
        $e_ki = $b_ki ^ ((!$b_ko) & $b_ku);
        $c_i ^= $e_ki;
        $e_ko = $b_ko ^ ((!$b_ku) & $b_ka);
        $c_o ^= $e_ko;
        $e_ku = $b_ku ^ ((!$b_ka) & $b_ke);
        $c_u ^= $e_ku;
        $a_bu ^= $d_u;
        $b_ma = $a_bu.rotate_left(27);
        $a_ga ^= $d_a;
        $b_me = $a_ga.rotate_left(36);
        $a_ke ^= $d_e;
        $b_mi = $a_ke.rotate_left(10);
        $a_mi ^= $d_i;
        $b_mo = $a_mi.rotate_left(15);
        $a_so ^= $d_o;
        $b_mu = $a_so.rotate_left(56);
        $e_ma = $b_ma ^ ((!$b_me) & $b_mi);
        $c_a ^= $e_ma;
        $e_me = $b_me ^ ((!$b_mi) & $b_mo);
        $c_e ^= $e_me;
        $e_mi = $b_mi ^ ((!$b_mo) & $b_mu);
        $c_i ^= $e_mi;
        $e_mo = $b_mo ^ ((!$b_mu) & $b_ma);
        $c_o ^= $e_mo;
        $e_mu = $b_mu ^ ((!$b_ma) & $b_me);
        $c_u ^= $e_mu;
        $a_bi ^= $d_i;
        $b_sa = $a_bi.rotate_left(62);
        $a_go ^= $d_o;
        $b_se = $a_go.rotate_left(55);
        $a_ku ^= $d_u;
        $b_si = $a_ku.rotate_left(39);
        $a_ma ^= $d_a;
        $b_so = $a_ma.rotate_left(41);
        $a_se ^= $d_e;
        $b_su = $a_se.rotate_left(2);
        $e_sa = $b_sa ^ ((!$b_se) & $b_si);
        $c_a ^= $e_sa;
        $e_se = $b_se ^ ((!$b_si) & $b_so);
        $c_e ^= $e_se;
        $e_si = $b_si ^ ((!$b_so) & $b_su);
        $c_i ^= $e_si;
        $e_so = $b_so ^ ((!$b_su) & $b_sa);
        $c_o ^= $e_so;
        $e_su = $b_su ^ ((!$b_sa) & $b_se);
        $c_u ^= $e_su;
        $d_a = $c_u ^ $c_e.rotate_left(1);
        $d_e = $c_a ^ $c_i.rotate_left(1);
        $d_i = $c_e ^ $c_o.rotate_left(1);
        $d_o = $c_i ^ $c_u.rotate_left(1);
        $d_u = $c_o ^ $c_a.rotate_left(1);
        $e_ba ^= $d_a;
        $b_ba = $e_ba;
        $e_ge ^= $d_e;
        $b_be = $e_ge.rotate_left(44);
        $e_ki ^= $d_i;
        $b_bi = $e_ki.rotate_left(43);
        $e_mo ^= $d_o;
        $b_bo = $e_mo.rotate_left(21);
        $e_su ^= $d_u;
        $b_bu = $e_su.rotate_left(14);
        $a_ba = $b_ba ^ ((!$b_be) & $b_bi);
        $a_ba ^= $rc_b;
        $c_a = $a_ba;
        $a_be = $b_be ^ ((!$b_bi) & $b_bo);
        $c_e = $a_be;
        $a_bi = $b_bi ^ ((!$b_bo) & $b_bu);
        $c_i = $a_bi;
        $a_bo = $b_bo ^ ((!$b_bu) & $b_ba);
        $c_o = $a_bo;
        $a_bu = $b_bu ^ ((!$b_ba) & $b_be);
        $c_u = $a_bu;
        $e_bo ^= $d_o;
        $b_ga = $e_bo.rotate_left(28);
        $e_gu ^= $d_u;
        $b_ge = $e_gu.rotate_left(20);
        $e_ka ^= $d_a;
        $b_gi = $e_ka.rotate_left(3);
        $e_me ^= $d_e;
        $b_go = $e_me.rotate_left(45);
        $e_si ^= $d_i;
        $b_gu = $e_si.rotate_left(61);
        $a_ga = $b_ga ^ ((!$b_ge) & $b_gi);
        $c_a ^= $a_ga;
        $a_ge = $b_ge ^ ((!$b_gi) & $b_go);
        $c_e ^= $a_ge;
        $a_gi = $b_gi ^ ((!$b_go) & $b_gu);
        $c_i ^= $a_gi;
        $a_go = $b_go ^ ((!$b_gu) & $b_ga);
        $c_o ^= $a_go;
        $a_gu = $b_gu ^ ((!$b_ga) & $b_ge);
        $c_u ^= $a_gu;
        $e_be ^= $d_e;
        $b_ka = $e_be.rotate_left(1);
        $e_gi ^= $d_i;
        $b_ke = $e_gi.rotate_left(6);
        $e_ko ^= $d_o;
        $b_ki = $e_ko.rotate_left(25);
        $e_mu ^= $d_u;
        $b_ko = $e_mu.rotate_left(8);
        $e_sa ^= $d_a;
        $b_ku = $e_sa.rotate_left(18);
        $a_ka = $b_ka ^ ((!$b_ke) & $b_ki);
        $c_a ^= $a_ka;
        $a_ke = $b_ke ^ ((!$b_ki) & $b_ko);
        $c_e ^= $a_ke;
        $a_ki = $b_ki ^ ((!$b_ko) & $b_ku);
        $c_i ^= $a_ki;
        $a_ko = $b_ko ^ ((!$b_ku) & $b_ka);
        $c_o ^= $a_ko;
        $a_ku = $b_ku ^ ((!$b_ka) & $b_ke);
        $c_u ^= $a_ku;
        $e_bu ^= $d_u;
        $b_ma = $e_bu.rotate_left(27);
        $e_ga ^= $d_a;
        $b_me = $e_ga.rotate_left(36);
        $e_ke ^= $d_e;
        $b_mi = $e_ke.rotate_left(10);
        $e_mi ^= $d_i;
        $b_mo = $e_mi.rotate_left(15);
        $e_so ^= $d_o;
        $b_mu = $e_so.rotate_left(56);
        $a_ma = $b_ma ^ ((!$b_me) & $b_mi);
        $c_a ^= $a_ma;
        $a_me = $b_me ^ ((!$b_mi) & $b_mo);
        $c_e ^= $a_me;
        $a_mi = $b_mi ^ ((!$b_mo) & $b_mu);
        $c_i ^= $a_mi;
        $a_mo = $b_mo ^ ((!$b_mu) & $b_ma);
        $c_o ^= $a_mo;
        $a_mu = $b_mu ^ ((!$b_ma) & $b_me);
        $c_u ^= $a_mu;
        $e_bi ^= $d_i;
        $b_sa = $e_bi.rotate_left(62);
        $e_go ^= $d_o;
        $b_se = $e_go.rotate_left(55);
        $e_ku ^= $d_u;
        $b_si = $e_ku.rotate_left(39);
        $e_ma ^= $d_a;
        $b_so = $e_ma.rotate_left(41);
        $e_se ^= $d_e;
        $b_su = $e_se.rotate_left(2);
        $a_sa = $b_sa ^ ((!$b_se) & $b_si);
        $c_a ^= $a_sa;
        $a_se = $b_se ^ ((!$b_si) & $b_so);
        $c_e ^= $a_se;
        $a_si = $b_si ^ ((!$b_so) & $b_su);
        $c_i ^= $a_si;
        $a_so = $b_so ^ ((!$b_su) & $b_sa);
        $c_o ^= $a_so;
        $a_su = $b_su ^ ((!$b_sa) & $b_se);
        $c_u ^= $a_su;
    };
}

macro_rules! copy_to_state {
    (
        $lanes:ident,
        $a_ba: ident, $a_be: ident, $a_bi: ident, $a_bo: ident, $a_bu: ident,
        $a_ga: ident, $a_ge: ident, $a_gi: ident, $a_go: ident, $a_gu: ident,
        $a_ka: ident, $a_ke: ident, $a_ki: ident, $a_ko: ident, $a_ku: ident,
        $a_ma: ident, $a_me: ident, $a_mi: ident, $a_mo: ident, $a_mu: ident,
        $a_sa: ident, $a_se: ident, $a_si: ident, $a_so: ident, $a_su: ident,
    ) => {
        $lanes[0] = $a_ba;
        $lanes[1] = $a_be;
        $lanes[2] = $a_bi;
        $lanes[3] = $a_bo;
        $lanes[4] = $a_bu;
        $lanes[5] = $a_ga;
        $lanes[6] = $a_ge;
        $lanes[7] = $a_gi;
        $lanes[8] = $a_go;
        $lanes[9] = $a_gu;
        $lanes[10] = $a_ka;
        $lanes[11] = $a_ke;
        $lanes[12] = $a_ki;
        $lanes[13] = $a_ko;
        $lanes[14] = $a_ku;
        $lanes[15] = $a_ma;
        $lanes[16] = $a_me;
        $lanes[17] = $a_mi;
        $lanes[18] = $a_mo;
        $lanes[19] = $a_mu;
        $lanes[20] = $a_sa;
        $lanes[21] = $a_se;
        $lanes[22] = $a_si;
        $lanes[23] = $a_so;
        $lanes[24] = $a_su;
    };
}

macro_rules! iter_rounds {
    ($lanes: ident, $( ($rc_a:tt, $rc_b:tt)),*) => {
        copy_from_state!(
            $lanes,
            a_ba, a_be, a_bi, a_bo, a_bu,
            a_ga, a_ge, a_gi, a_go, a_gu,
            a_ka, a_ke, a_ki,a_ko, a_ku,
            a_ma, a_me, a_mi, a_mo, a_mu,
            a_sa, a_se, a_si, a_so, a_su,
            b_ba, b_be, b_bi, b_bo, b_bu,
            b_ga, b_ge, b_gi, b_go, b_gu,
            b_ka, b_ke, b_ki, b_ko, b_ku,
            b_ma, b_me, b_mi, b_mo, b_mu,
            b_sa, b_se, b_si, b_so, b_su,
            c_a, c_e, c_i, c_o, c_u,
            d_a, d_e, d_i, d_o, d_u,
            e_ba, e_be, e_bi, e_bo, e_bu,
            e_ga, e_ge, e_gi, e_go, e_gu,
            e_ka, e_ke, e_ki, e_ko, e_ku,
            e_ma, e_me, e_mi, e_mo, e_mu,
            e_sa, e_se, e_si, e_so, e_su,
        );
        $(
            double_round!(
                $rc_a, $rc_b,
                a_ba, a_be, a_bi, a_bo, a_bu,
                a_ga, a_ge, a_gi, a_go, a_gu,
                a_ka, a_ke, a_ki, a_ko, a_ku,
                a_ma, a_me, a_mi, a_mo, a_mu,
                a_sa, a_se, a_si, a_so, a_su,
                b_ba, b_be, b_bi, b_bo, b_bu,
                b_ga, b_ge, b_gi, b_go, b_gu,
                b_ka, b_ke, b_ki, b_ko, b_ku,
                b_ma, b_me, b_mi, b_mo, b_mu,
                b_sa, b_se, b_si, b_so, b_su,
                c_a, c_e, c_i, c_o, c_u,
                d_a, d_e, d_i,d_o, d_u,
                e_ba, e_be, e_bi, e_bo, e_bu,
                e_ga, e_ge, e_gi, e_go, e_gu,
                e_ka, e_ke, e_ki, e_ko, e_ku,
                e_ma, e_me, e_mi, e_mo, e_mu,
                e_sa, e_se, e_si, e_so, e_su,
            );
        )*
        copy_to_state!(
            $lanes,
            a_ba, a_be, a_bi, a_bo, a_bu,
            a_ga, a_ge, a_gi, a_go, a_gu,
            a_ka, a_ke, a_ki,a_ko, a_ku,
            a_ma, a_me, a_mi, a_mo, a_mu,
            a_sa, a_se, a_si, a_so, a_su,
        );
    };
}
