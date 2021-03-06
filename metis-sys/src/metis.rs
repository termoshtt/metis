/* automatically generated by rust-bindgen 0.54.1 */

pub type __int32_t = ::std::os::raw::c_int;
pub type idx_t = i32;
pub type real_t = f32;
extern "C" {
    pub fn METIS_PartGraphRecursive(
        nvtxs: *mut idx_t,
        ncon: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
        options: *mut idx_t,
        edgecut: *mut idx_t,
        part: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_PartGraphKway(
        nvtxs: *mut idx_t,
        ncon: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        adjwgt: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        ubvec: *mut real_t,
        options: *mut idx_t,
        edgecut: *mut idx_t,
        part: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_MeshToDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        ncommon: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_MeshToNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        numflag: *mut idx_t,
        r_xadj: *mut *mut idx_t,
        r_adjncy: *mut *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_PartMeshNodal(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_PartMeshDual(
        ne: *mut idx_t,
        nn: *mut idx_t,
        eptr: *mut idx_t,
        eind: *mut idx_t,
        vwgt: *mut idx_t,
        vsize: *mut idx_t,
        ncommon: *mut idx_t,
        nparts: *mut idx_t,
        tpwgts: *mut real_t,
        options: *mut idx_t,
        objval: *mut idx_t,
        epart: *mut idx_t,
        npart: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_NodeND(
        nvtxs: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        options: *mut idx_t,
        perm: *mut idx_t,
        iperm: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_Free(ptr: *mut ::core::ffi::c_void) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_SetDefaultOptions(options: *mut idx_t) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_NodeNDP(
        nvtxs: idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        npes: idx_t,
        options: *mut idx_t,
        perm: *mut idx_t,
        iperm: *mut idx_t,
        sizes: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_ComputeVertexSeparator(
        nvtxs: *mut idx_t,
        xadj: *mut idx_t,
        adjncy: *mut idx_t,
        vwgt: *mut idx_t,
        options: *mut idx_t,
        sepsize: *mut idx_t,
        part: *mut idx_t,
    ) -> ::std::os::raw::c_int;
}
extern "C" {
    pub fn METIS_NodeRefine(
        nvtxs: idx_t,
        xadj: *mut idx_t,
        vwgt: *mut idx_t,
        adjncy: *mut idx_t,
        where_: *mut idx_t,
        hmarker: *mut idx_t,
        ubfactor: real_t,
    ) -> ::std::os::raw::c_int;
}
#[repr(i32)]
#[doc = " Return codes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum rstatus_et {
    #[doc = "< Returned normally"]
    METIS_OK = 1,
    #[doc = "< Returned due to erroneous inputs and/or options"]
    METIS_ERROR_INPUT = -2,
    #[doc = "< Returned due to insufficient memory"]
    METIS_ERROR_MEMORY = -3,
    #[doc = "< Some other errors"]
    METIS_ERROR = -4,
}
#[repr(u32)]
#[doc = " Operation type codes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum moptype_et {
    METIS_OP_PMETIS = 0,
    METIS_OP_KMETIS = 1,
    METIS_OP_OMETIS = 2,
}
#[repr(u32)]
#[doc = " Options codes (i.e., options[])"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum moptions_et {
    METIS_OPTION_PTYPE = 0,
    METIS_OPTION_OBJTYPE = 1,
    METIS_OPTION_CTYPE = 2,
    METIS_OPTION_IPTYPE = 3,
    METIS_OPTION_RTYPE = 4,
    METIS_OPTION_DBGLVL = 5,
    METIS_OPTION_NITER = 6,
    METIS_OPTION_NCUTS = 7,
    METIS_OPTION_SEED = 8,
    METIS_OPTION_NO2HOP = 9,
    METIS_OPTION_MINCONN = 10,
    METIS_OPTION_CONTIG = 11,
    METIS_OPTION_COMPRESS = 12,
    METIS_OPTION_CCORDER = 13,
    METIS_OPTION_PFACTOR = 14,
    METIS_OPTION_NSEPS = 15,
    METIS_OPTION_UFACTOR = 16,
    METIS_OPTION_NUMBERING = 17,
    METIS_OPTION_HELP = 18,
    METIS_OPTION_TPWGTS = 19,
    METIS_OPTION_NCOMMON = 20,
    METIS_OPTION_NOOUTPUT = 21,
    METIS_OPTION_BALANCE = 22,
    METIS_OPTION_GTYPE = 23,
    METIS_OPTION_UBVEC = 24,
}
#[repr(u32)]
#[doc = " Partitioning Schemes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mptype_et {
    METIS_PTYPE_RB = 0,
    METIS_PTYPE_KWAY = 1,
}
#[repr(u32)]
#[doc = " Graph types for meshes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mgtype_et {
    METIS_GTYPE_DUAL = 0,
    METIS_GTYPE_NODAL = 1,
}
#[repr(u32)]
#[doc = " Coarsening Schemes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mctype_et {
    METIS_CTYPE_RM = 0,
    METIS_CTYPE_SHEM = 1,
}
#[repr(u32)]
#[doc = " Initial partitioning schemes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum miptype_et {
    METIS_IPTYPE_GROW = 0,
    METIS_IPTYPE_RANDOM = 1,
    METIS_IPTYPE_EDGE = 2,
    METIS_IPTYPE_NODE = 3,
    METIS_IPTYPE_METISRB = 4,
}
#[repr(u32)]
#[doc = " Refinement schemes"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mrtype_et {
    METIS_RTYPE_FM = 0,
    METIS_RTYPE_GREEDY = 1,
    METIS_RTYPE_SEP2SIDED = 2,
    METIS_RTYPE_SEP1SIDED = 3,
}
#[repr(u32)]
#[doc = " Debug Levels"]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mdbglvl_et {
    #[doc = "< Shows various diagnostic messages"]
    METIS_DBG_INFO = 1,
    #[doc = "< Perform timing analysis"]
    METIS_DBG_TIME = 2,
    #[doc = "< Show the coarsening progress"]
    METIS_DBG_COARSEN = 4,
    #[doc = "< Show the refinement progress"]
    METIS_DBG_REFINE = 8,
    #[doc = "< Show info on initial partitioning"]
    METIS_DBG_IPART = 16,
    #[doc = "< Show info on vertex moves during refinement"]
    METIS_DBG_MOVEINFO = 32,
    #[doc = "< Show info on vertex moves during sep refinement"]
    METIS_DBG_SEPINFO = 64,
    #[doc = "< Show info on minimization of subdomain connectivity"]
    METIS_DBG_CONNINFO = 128,
    #[doc = "< Show info on elimination of connected components"]
    METIS_DBG_CONTIGINFO = 256,
    #[doc = "< Show info related to wspace allocation"]
    METIS_DBG_MEMORY = 2048,
}
#[repr(u32)]
#[derive(FromPrimitive, ToPrimitive, Debug, Copy, Clone, PartialEq, Eq, Hash, PartialOrd, Ord)]
pub enum mobjtype_et {
    METIS_OBJTYPE_CUT = 0,
    METIS_OBJTYPE_VOL = 1,
    METIS_OBJTYPE_NODE = 2,
}
