use crate::binary::*;

use int_enum::IntEnum;

use super::ResourceReturnType;

bitflags! {
    pub struct ShaderInputFlags: u32 {
        const USER_PACKED = 0x1;
        const COMPARISON_SAMPLER = 0x2;
        const TEXTURE_COMPONENT_0 = 0x4;
        const TEXTURE_COMPONENT_1 = 0x8;
        const TEXTURE_COMPONENTS = 0xc;
        const UNUSED = 0x10;
    }

    pub struct ShaderVariableFlags: u32 {
        const USER_PACKED = 0x1;
        const USED = 0x2;
        const INTERFACE_POINTER = 0x4;
        const INTERFACE_PARAMETER = 0x8;
    }

    pub struct ConstantBufferFlags: u32 {
        const USER_PACKED = 0x1;
    }
}

#[repr(u32)]
#[derive(Debug)]
pub enum ConstantBufferType {
    ConstantBuffer,
    TextureBuffer,
    InterfacePointers,
    ResourceBindInformation,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ShaderInputType {
    CBuffer = 0,
    TBuffer = 1,
    Texture = 2,
    Sampler = 3,
    UavRwTyped = 4,
    Structured = 5,
    UavRwStructured = 6,
    ByteAddress = 7,
    UavRwByteAddress = 8,
    UavAppendStructured = 9,
    UavConsumeStructured = 10,
    UavRwStructuredWithCounter = 11,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ShaderVariableClass {
    Scalar = 0,
    Vector = 1,
    MatrixRows = 2,
    MatrixColumns = 3,
    Object = 4,
    Struct = 5,
    InterfaceClass = 6,
    InterfacePointer = 7,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ShaderVariableType {
    Void = 0,
    Bool = 1,
    Int_ = 2,
    Float = 3,
    String = 4,
    Texture = 5,
    Texture1D = 6,
    Texture2D = 7,
    Texture3D = 8,
    TextureCube = 9,
    Sampler = 10,
    PixelShader = 15,
    VertexShader = 16,
    UInt = 19,
    UInt8 = 20,
    GeometryShader = 21,
    Rasterizer = 22,
    DepthStencil = 23,
    Blend = 24,
    Buffer = 25,
    CBuffer = 26,
    TBuffer = 27,
    Texture1DArray = 28,
    Texture2DArray = 29,
    RenderTargetView = 30,
    DepthStencilView = 31,
    Texture2DMultiSampled = 32,
    Texture2DMultiSampledArray = 33,
    TextureCubeArray = 34,
    HullShader = 35,
    DomainShader = 36,
    InterfacePointer = 37,
    ComputeShader = 38,
    Double = 39,
    ReadWriteTexture1D = 40,
    ReadWriteTexture1DArray = 41,
    ReadWriteTexture2D = 42,
    ReadWriteTexture2DArray = 43,
    ReadWriteTexture3D = 44,
    ReadWriteBuffer = 45,
    ByteAddressBuffer = 46,
    ReadWriteByteAddressBuffer = 47,
    StructuredBuffer = 48,
    ReadWriteStructuredBuffer = 49,
    AppendStructuredBuffer = 50,
    ConsumeStructuredBuffer = 51,
}

#[repr(u32)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ViewDimension {
    Unknown = 0,
    Buffer = 1,
    Texture1D = 2,
    Texture1DArray = 3,
    Texture2D = 4,
    Texture2DArray = 5,
    Texture2DMultiSampled = 6,
    Texture2DMultiSampledArray = 7,
    Texture3D = 8,
    TextureCube = 9,
    TextureCubeArray = 10,
    ExtendedBuffer = 11,
}

#[repr(u32)]
#[derive(Debug)]
pub enum ShaderModel {
    V5_0,
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderTypeMember<'a> {
    name: &'a str,
    ty: ShaderType<'a>,
    offset: u32,
}

impl<'a> ShaderTypeMember<'a> {
    pub fn parse(decoder: &mut Decoder<'a>, major: u8) -> Result<Self, State> {
        let name_offset = decoder.read_u32() as usize;
        let ty_offset = decoder.read_u32() as usize;
        let offset = decoder.read_u32();

        let name = decoder
            .seek(name_offset)
            .str()
            .map_err(State::DecoderError)?;
        let ty = ShaderType::parse(&mut decoder.seek(ty_offset), major)?;

        Ok(Self { name, ty, offset })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderType<'a> {
    pub class: ShaderVariableClass,
    pub ty: ShaderVariableType,
    pub rows: u16,
    pub columns: u16,
    pub count: u16,
    pub members: Vec<ShaderTypeMember<'a>>,
    pub parent_ty_class: Option<ShaderVariableClass>,
    pub parent_name: Option<&'a str>,
    pub unknowns: [Option<u32>; 4],
}

impl<'a> ShaderType<'a> {
    pub fn parse(decoder: &mut Decoder<'a>, major: u8) -> Result<Self, State> {
        let class = read_enum!(ShaderVariableClass, decoder, u16);
        let ty = read_enum!(ShaderVariableType, decoder, u16);
        let rows = decoder.read_u16();
        let columns = decoder.read_u16();
        let count = decoder.read_u16();
        let member_count = decoder.read_u16();
        let member_offset = decoder.read_u32() as usize;

        let mut parent_ty_class = None;
        let mut parent_name = None;
        let mut unknowns = [None; 4];

        if major >= 5 {
            let parent_ty_offset = decoder.read_u32() as usize;
            if parent_ty_offset != 0 {
                let mut parent_ty_decoder = decoder.seek(parent_ty_offset);
                parent_ty_class = Some(read_enum!(ShaderVariableClass, parent_ty_decoder, u16));
                unknowns[0] = Some(parent_ty_decoder.read_u16() as u32);
            }

            let unknown_2_offset = decoder.read_u32();
            if unknown_2_offset != 0 {
                unknowns[1] = Some(decoder.seek(unknown_2_offset as usize).read_u32());
            }

            unknowns[2] = Some(decoder.read_u32());

            let unknown_5_offset = decoder.read_u32();
            if unknown_5_offset != 0 {
                unknowns[3] = Some(decoder.seek(unknown_5_offset as usize).read_u32());
            }

            let parent_name_offset = decoder.read_u32() as usize;
            if parent_name_offset != 0 {
                parent_name = Some(
                    decoder
                        .seek(parent_name_offset)
                        .str()
                        .map_err(State::DecoderError)?,
                );
            }
        }

        let mut members = Vec::with_capacity(member_count.into());
        decoder.seek_mut(member_offset);
        for _ in 0..member_count {
            members.push(ShaderTypeMember::parse(decoder, major)?);
        }

        Ok(Self {
            class,
            ty,
            rows,
            columns,
            count,
            members,
            parent_ty_class,
            parent_name,
            unknowns,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ShaderVariable<'a> {
    pub name: &'a str,
    pub offset: u32,
    pub size: u32,
    pub flags: ShaderVariableFlags,
    pub ty: ShaderType<'a>,
    pub default_value: Vec<&'a [u8]>,
    pub start_texture: Option<u32>,
    pub texture_size: Option<u32>,
    pub start_sampler: Option<u32>,
    pub sampler_size: Option<u32>,
}

impl<'a> ShaderVariable<'a> {
    pub fn parse(decoder: &mut Decoder<'a>, major: u8) -> Result<Self, State> {
        let name_offset = decoder.read_u32() as usize;
        let offset = decoder.read_u32();
        let size = decoder.read_u32();
        let flags = ShaderVariableFlags::from_bits_truncate(decoder.read_u32());
        let ty_offset = decoder.read_u32() as usize;
        let default_offset = decoder.read_u32() as usize;

        let name = decoder
            .seek(name_offset)
            .str()
            .map_err(State::DecoderError)?;
        let ty = ShaderType::parse(&mut decoder.seek(ty_offset), major)?;
        // TODO: figure out scenarios that could cause non-4-divisible default values
        let values_count = (size / 4) as usize;
        let mut default_value = Vec::with_capacity(values_count);
        if default_offset != 0 {
            for _ in 0..values_count {
                default_value.push(decoder.bytes(4));
            }
        }

        let (start_texture, texture_size, start_sampler, sampler_size) = if major >= 5 {
            (
                Some(decoder.read_u32()),
                Some(decoder.read_u32()),
                Some(decoder.read_u32()),
                Some(decoder.read_u32()),
            )
        } else {
            (None, None, None, None)
        };

        Ok(Self {
            name,
            offset,
            size,
            flags,
            ty,
            default_value,
            start_texture,
            texture_size,
            start_sampler,
            sampler_size,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ConstantBuffer<'a> {
    pub name: &'a str,
    pub variables: Vec<ShaderVariable<'a>>,
    pub size: u32,
    pub flags: u32,
    pub ty: u32,
}

impl<'a> ConstantBuffer<'a> {
    pub fn parse(decoder: &mut Decoder<'a>, major: u8) -> Result<Self, State> {
        let name_offset = decoder.read_u32() as usize;
        let var_count = decoder.read_u32() as usize;
        let var_offset = decoder.read_u32() as usize;
        let size = decoder.read_u32();
        let flags = decoder.read_u32();
        let ty = decoder.read_u32();

        let name = decoder
            .seek(name_offset)
            .str()
            .map_err(State::DecoderError)?;
        
        let mut variables = Vec::with_capacity(var_count);
        let mut var_decoder = decoder.seek(var_offset);
        for _ in 0..var_count {
            variables.push(ShaderVariable::parse(&mut var_decoder, major)?);
        }

        Ok(Self {
            name,
            variables,
            size,
            flags,
            ty,
        })
    }
}

#[repr(C)]
#[derive(Debug)]
pub struct ResourceBinding<'a> {
    pub name: &'a str,
    pub input_type: ShaderInputType,
    pub return_type: ResourceReturnType,
    pub view_dimension: ViewDimension,
    pub sample_count: u32,
    pub bind_point: u32,
    pub bind_count: u32,
    pub input_flags: ShaderInputFlags,
}

impl<'a> ResourceBinding<'a> {
    pub fn parse(decoder: &mut Decoder<'a>) -> Result<Self, State> {
        let name_offset = decoder.read_u32();
        let input_type = read_enum!(ShaderInputType, decoder, u32);
        let return_type = read_enum!(ResourceReturnType, decoder, u32);
        let view_dimension = read_enum!(ViewDimension, decoder, u32);
        let sample_count = decoder.read_u32();
        let bind_point = decoder.read_u32();
        let bind_count = decoder.read_u32();
        let input_flags = ShaderInputFlags::from_bits_truncate(decoder.read_u32());

        let name = decoder
            .seek(name_offset as usize)
            .str()
            .map_err(State::DecoderError)?;

        Ok(Self {
            name,
            input_type,
            return_type,
            view_dimension,
            sample_count,
            bind_point,
            bind_count,
            input_flags,
        })
    }
}

#[repr(u16)]
#[derive(Clone, Copy, Debug, IntEnum)]
pub enum ProgramType {
    Pixel = 0xFFFF,
    Vertex = 0xFFFE,
    Hull = 0x4853,
    Geometry = 0x4753,
    Domain = 0x4453,
    Compute = 0x4353,
}

#[repr(C)]
#[derive(Debug)]
pub struct RdefChunk<'a> {
    pub constant_buffers: Vec<ConstantBuffer<'a>>,
    pub resource_bindings: Vec<ResourceBinding<'a>>,
    pub program_ty: ProgramType,
    pub minor: u8,
    pub major: u8,
    pub flags: u32,
    pub author: &'a str,
    pub rd11: Option<[u32; 7]>,
}

impl<'a> RdefChunk<'a> {
    pub fn parse<'b>(decoder: &'b mut Decoder) -> Result<RdefChunk<'b>, State> {
        let cb_count = decoder.read_u32() as usize;
        let cb_offset = decoder.read_u32() as usize;

        let bind_count = decoder.read_u32() as usize;
        let bind_offset = decoder.read_u32() as usize;

        let minor = decoder.read_u8();
        let major = decoder.read_u8();

        let program_ty = read_enum!(ProgramType, decoder, u16);
        let flags = decoder.read_u32();
        let author_offset = decoder.read_u32();

        let rd11 = if major >= 5 {
            let _magic = decoder.read_u32();
            // assert_eq!(magic, b"RD11");

            Some([
                decoder.read_u32(),
                decoder.read_u32(),
                decoder.read_u32(),
                decoder.read_u32(),
                decoder.read_u32(),
                decoder.read_u32(),
                decoder.read_u32(),
            ])
        } else {
            None
        };

        decoder.seek_mut(cb_offset);
        let mut constant_buffers = Vec::with_capacity(cb_count);
        for _ in 0..cb_count {
            constant_buffers.push(ConstantBuffer::parse(decoder, major)?);
        }

        decoder.seek_mut(bind_offset);
        let mut resource_bindings = Vec::with_capacity(bind_count);
        for _ in 0..bind_count {
            resource_bindings.push(ResourceBinding::parse(decoder)?);
        }

        let author = decoder
            .seek(author_offset as usize)
            .str()
            .map_err(State::DecoderError)?;

        Ok(RdefChunk {
            constant_buffers,
            resource_bindings,
            program_ty,
            minor,
            major,
            flags,
            author,
            rd11,
        })
    }
}
