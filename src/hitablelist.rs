use vec3::ElemT;
use ray::Ray;
use hitable::{HitRecord, Hitable};

type ListT<T> = Vec<Box<Hitable<T>>>;

#[derive(Default)]
pub struct HitableList<T>
    where T: ElemT {
    list : ListT<T>
}

impl<T: ElemT> HitableList<T> {
    pub fn new(v: Vec<Box<Hitable<T>>>) -> HitableList<T> {
        HitableList::<T> {
            list: v
        }
    }
}

impl<T: ElemT> Hitable<T> for HitableList<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T) -> Option<HitRecord<T>> {
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
