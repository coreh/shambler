use std::collections::BTreeMap;

use shalrath::repr::{
    Brush, BrushPlane, Entity, Extension, Properties, TextureOffset, TrianglePlane,
};
use usage::Usage;

use crate::{brush::BrushId, entity::EntityId, face::FaceId, texture::TextureId, Vector2};

pub enum EntitiesTag {}
pub enum BrushesTag {}
pub enum FacesTag {}
pub enum PointEntitiesTag {}
pub enum EntityPropertiesTag {}
pub enum EntityBrushesTag {}
pub enum BrushFacesTag {}
pub enum FaceTrianglePlanesTag {}
pub enum FaceTexturesTag {}
pub enum FaceOffsetsTag {}
pub enum FaceAnglesTag {}
pub enum FaceScalesTag {}
pub enum FaceExtensionsTag {}
pub enum TexturesTag {}

pub type Entities = Usage<EntitiesTag, Vec<EntityId>>;
pub type Brushes = Usage<BrushesTag, Vec<BrushId>>;
pub type Faces = Usage<FacesTag, Vec<FaceId>>;

pub type PointEntities = Usage<PointEntitiesTag, Vec<EntityId>>;

pub type EntityProperties = Usage<EntityPropertiesTag, BTreeMap<EntityId, Properties>>;
pub type EntityBrushes = Usage<EntityBrushesTag, BTreeMap<EntityId, Vec<BrushId>>>;

pub type BrushFaces = Usage<BrushFacesTag, BTreeMap<BrushId, Vec<FaceId>>>;

pub type FaceTrianglePlanes = Usage<FaceTrianglePlanesTag, BTreeMap<FaceId, TrianglePlane>>;
pub type FaceTextures = Usage<FaceTexturesTag, BTreeMap<FaceId, TextureId>>;
pub type FaceOffsets = Usage<FaceOffsetsTag, BTreeMap<FaceId, TextureOffset>>;
pub type FaceAngles = Usage<FaceAnglesTag, BTreeMap<FaceId, f64>>;
pub type FaceScales = Usage<FaceScalesTag, BTreeMap<FaceId, Vector2>>;
pub type FaceExtensions = Usage<FaceExtensionsTag, BTreeMap<FaceId, Extension>>;

pub type Textures = Usage<TexturesTag, BTreeMap<TextureId, String>>;

/// Struct-of-arrays representation of a [`shalrath::repr::Map`]
#[derive(Debug, Default, Clone)]
pub struct GeoMap {
    pub entities: Entities,
    pub brushes: Brushes,
    pub faces: Faces,

    pub textures: Textures,

    pub entity_properties: EntityProperties,
    pub entity_brushes: EntityBrushes,
    pub point_entities: PointEntities,

    pub brush_faces: BrushFaces,

    pub face_planes: FaceTrianglePlanes,
    pub face_textures: FaceTextures,
    pub face_offsets: FaceOffsets,
    pub face_angles: FaceAngles,
    pub face_scales: FaceScales,
    pub face_extensions: FaceExtensions,
}

impl GeoMap {
    pub fn new(shalrath::repr::Map(map): shalrath::repr::Map) -> Self {
        let mut entity_head = 0;
        let mut brush_head = 0;
        let mut plane_head = 0;
        let mut texture_head = 0;

        let mut entities = Entities::default();
        let mut brushes = Brushes::default();
        let mut faces = Faces::default();

        let mut entity_properties = EntityProperties::default();
        let mut entity_brushes = EntityBrushes::default();

        let mut brush_faces = BrushFaces::default();

        let mut face_planes = FaceTrianglePlanes::default();
        let mut face_textures = FaceTextures::default();
        let mut face_offsets = FaceOffsets::default();
        let mut face_angles = FaceAngles::default();
        let mut face_scales = FaceScales::default();
        let mut face_extensions = FaceExtensions::default();

        let mut textures = BTreeMap::<String, TextureId>::new();

        for Entity {
            properties,
            brushes: shalrath::repr::Brushes(bs),
        } in map.into_iter()
        {
            let entity_id = EntityId(entity_head);
            entity_head += 1;

            entities.push(entity_id);
            entity_properties.insert(entity_id, properties);

            for Brush(ps) in bs {
                let brush_id = BrushId(brush_head);
                brush_head += 1;

                brushes.push(brush_id);
                entity_brushes.entry(entity_id).or_default().push(brush_id);

                for BrushPlane {
                    plane,
                    texture,
                    texture_offset,
                    angle,
                    scale_x,
                    scale_y,
                    extension,
                } in ps
                {
                    let plane_id = FaceId(plane_head);
                    plane_head += 1;

                    faces.push(plane_id);
                    face_planes.insert(plane_id, plane);

                    let texture_id = if let Some(texture_id) = textures.get(&texture) {
                        *texture_id
                    } else {
                        let texture_id = TextureId(texture_head);
                        textures.insert(texture, texture_id);
                        texture_head += 1;
                        texture_id
                    };

                    face_textures.insert(plane_id, texture_id);

                    face_offsets.insert(plane_id, texture_offset);
                    face_angles.insert(plane_id, angle as f64);
                    face_scales.insert(plane_id, nalgebra::vector![scale_x as f64, scale_y as f64]);
                    face_extensions.insert(plane_id, extension);
                    brush_faces.entry(brush_id).or_default().push(plane_id);
                }
            }
        }

        let point_entities = entities
            .iter()
            .filter(|entity_id| !entity_brushes.contains_key(entity_id))
            .copied()
            .collect();

        let textures = textures
            .into_iter()
            .map(|(k, v)| (v, k))
            .collect();

        GeoMap {
            entities,
            brushes,
            faces,
            textures,
            entity_properties,
            entity_brushes,
            point_entities,
            brush_faces,
            face_planes,
            face_textures,
            face_offsets,
            face_angles,
            face_scales,
            face_extensions,
        }
    }
}

impl From<shalrath::repr::Map> for GeoMap {
    fn from(map: shalrath::repr::Map) -> Self {
        GeoMap::new(map)
    }
}
