use vec3::ElemT;
use ray::Ray;
use hitable::{HitRecord, Hitable};

type ListT<T> = Vec<Box<Hitable<T>>>;
pub struct HitableList<T: ElemT> {
    list : ListT<T>
}

impl<T: ElemT> HitableList<T> {
    pub fn new(v: Vec<Box<Hitable<T>>>) -> HitableList<T> {
        HitableList::<T> {
            list: v
        }
    }
}

impl<T: ElemT> Default for HitableList<T> {
    fn default() -> HitableList<T> {
        HitableList::<T> {
            list: ListT::<T>::new()
        }
    }
}

impl<T: ElemT> Hitable<T> for HitableList<T> {
    fn hit(&self, r: &Ray<T>, t_min: T, t_max: T, rec: &mut HitRecord<T>) -> bool {
        let mut temp_rec = HitRecord::<T>::default();
        let mut hit_anything = false;
        let mut closest_so_far = t_max;
        for ref h in &self.list {
            if (*h).hit(&r, t_min, closest_so_far, &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                rec.clone_from(&temp_rec);
            }
        }
        hit_anything
    }
}
