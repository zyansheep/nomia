use super::*;
use std::ptr::NonNull;
use std::vec::Vec;

impl Name_ {
    pub fn to_rust(&self) -> OwnedName {
        let self_ = unsafe { &self.u.letin_ };
        OwnedName {
            first_declarations: match self_.listdeclaration_.as_ref() {
                Option::None => vec![],
                Option::Some(decls) => decls.to_rust(),
            },
            last_declaration: NonNull::new_unchecked(self_.substitution_)
                .as_ref()
                .to_rust(),
        }
    }
}

impl ListDeclaration_ {
    pub fn len(&self) -> usize {
        let mut res = 1;
        let mut me = &self;
        loop {
            match me.listdeclaration_.as_ref() {
                Option::None => return res,
                Option::Some(tl) => {
                    res += 1;
                    me = &tl;
                }
            }
        }
    }

    pub fn to_rust(&self) -> Vec<Box<super::Declaration<OwnedNameParameters>>> {
        let mut res = Vec::with_capacity(self.len());
        unimplemented!();
        res
    }
}

impl Declaration_ {
    pub fn to_rust(&self) -> Box<super::Declaration<OwnedNameParameters>> {
        unimplemented!();
    }
}
