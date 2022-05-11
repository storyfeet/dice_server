use err_tools::*;
use std::collections::BTreeMap;

pub struct QueryParams<'a> {
    sp: std::str::Split<'a, char>,
}

impl<'a> QueryParams<'a> {
    pub fn new(s: &'a str) -> Self {
        Self { sp: s.split('&') }
    }
}

fn hexy(c: char) -> anyhow::Result<u32> {
    match c {
        '0'..='9' => Ok(c as u32 - '0' as u32),
        'A'..='F' => Ok(c as u32 - 'A' as u32 + 10),
        'a'..='f' => Ok(c as u32 - 'a' as u32 + 10),
        _ => e_str("Not a hex string"),
    }
}

fn parse_percent(it: &mut std::str::Chars) -> anyhow::Result<char> {
    let mut cnum = hexy(it.next().e_str("no chars after percent")?)? * 16;
    cnum += hexy(it.next().e_str("no chars after percent")?)?;
    Ok(char::from_u32(cnum).e_str("No char from those numbers")?)
}

fn parse_param(s: &str) -> anyhow::Result<String> {
    let mut res = String::new();
    let mut it = s.chars();
    while let Some(c) = it.next() {
        match c {
            '%' => res.push(parse_percent(&mut it)?),
            c => res.push(c),
        }
    }
    Ok(res)
}

impl<'a> Iterator for QueryParams<'a> {
    type Item = (String, String);
    fn next(&mut self) -> Option<Self::Item> {
        let mut sp2 = self.sp.next()?.split('=');
        let k = sp2.next()?;
        let v = sp2.next()?;

        Some((parse_param(k).ok()?, parse_param(v).ok()?))
    }
}

pub struct QueryMap {
    pub map: BTreeMap<String, String>,
}

impl QueryMap {
    pub fn new(s: &str) -> Self {
        let mut map = BTreeMap::new();
        for (k, v) in QueryParams::new(s) {
            map.insert(k, v);
        }
        Self { map }
    }

    pub fn get<'a>(&'a self, s: &str) -> Option<&'a str> {
        self.map.get(s).map(String::as_str)
    }
}
