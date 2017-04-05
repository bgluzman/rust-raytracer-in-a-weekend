use vec3::ElemT;
use ray::Ray;
use hitable::{HitRecord, Hitable};

type ListT<'a, T> = Vec<Box<Hitable<'a, T>>>;
pub struct HitableList<'a, T>
    where T: ElemT + 'a {
    list : ListT<'a, T>
}

impl<'a, T:'a + ElemT> HitableList<'a, T> {
    pub fn new(v: Vec<Box<Hitable<T>>>) -> HitableList<T> {
        HitableList::<T> {
            list: v
        }
    }
}

impl<'a, T:'a + ElemT> Default for HitableList<'a, T> {
    fn default() -> HitableList<'a, T> {
        HitableList::<T> {
            list: ListT::<T>::new()
        }
    }
}

impl<'a, T: ElemT> Hitable<'a, T> for HitableList<'a, T> {
    fn hit(&'a self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<'a, T>> {
        let mut ret: Option<HitRecord<T>> = None;
        let mut closest_so_far = t_max;
        for ref h in &self.list {
            if let Some(rec) = (*h).hit(&r, t_min, closest_so_far) {
                closest_so_far = rec.t;
                ret = Some(rec);
            }
        }
        ret
    }
}
