#[derive(Default, Debug, Clone, PartialEq, serde_derive::Serialize, serde_derive::Deserialize)]
pub struct ArxivPagesLinksStruct {
    pub Astrophysics: Astrophysics,
    pub CondensedMatter: CondensedMatter,
    pub NonlinearSciences: NonlinearSciences,
    pub ComputingResearchRepository: ComputingResearchRepository,
    pub QuantitativeBiology: QuantitativeBiology,
    pub Mathematics: Mathematics,
    pub Statistics: Statistics,
    pub ElectricalEngineeringAndSystemsScience: ElectricalEngineeringAndSystemsScience,
    pub Physics: Physics,
    pub GeneralRelativityAndQuantumCosmology: String,
    pub HighEnergyPhysicsExperiment: String,
    pub HighEnergyPhysicsLattice: String,
    pub HighEnergyPhysicsPhenomenology: String,
    pub HighEnergyPhysicsTheory: String,
    pub MathematicalPhysics: String,
    pub NuclearExperiment: String,
    pub NuclearTheory: String,
    pub QuantumPhysics: String,
}

impl ArxivPagesStruct {
    pub fn new() -> Self {
        ArxivPagesStruct {
            Astrophysics: Astrophysics::new(),
            CondensedMatter: CondensedMatter::new(),
            NonlinearSciences: NonlinearSciences::new(),
            ComputingResearchRepository: ComputingResearchRepository::new(),
            QuantitativeBiology: QuantitativeBiology::new(),
            Mathematics: Mathematics::new(),
            Statistics: Statistics::new(),
            ElectricalEngineeringAndSystemsScience: ElectricalEngineeringAndSystemsScience::new(),
            Physics: Physics::new(),
            GeneralRelativityAndQuantumCosmology: "gr-qc".to_string(),
            HighEnergyPhysicsExperiment: "hep-ex".to_string(),
            HighEnergyPhysicsLattice: "hep-lat".to_string(),
            HighEnergyPhysicsPhenomenology: "hep-ph".to_string(),
            HighEnergyPhysicsTheory: "hep-th".to_string(),
            MathematicalPhysics: "math-ph".to_string(),
            NuclearExperiment: "nucl-ex".to_string(),
            NuclearTheory: "nucl-th".to_string(),
            QuantumPhysics: "quant-ph".to_string(),
        }
    }
}

pub struct Astrophysics {
    pub AstrophysicsOfGalaxies: String,
    pub CosmologyAndNongalacticAstrophysics: String,
    pub EarthAndPlanetaryAstrophysics: String,
    pub HighEnergyAstrophysicalPhenomena: String,
    pub InstrumentationAndMethodsForAstrophysics: String,
    pub SolarAndStellarAstrophysics: String,
}

impl Astrophysics {
    pub fn new() -> Self {
        Astrophysics {
            AstrophysicsOfGalaxies: "".to_string(),
            CosmologyAndNongalacticAstrophysics: "".to_string(),
            EarthAndPlanetaryAstrophysics: "".to_string(),
            HighEnergyAstrophysicalPhenomena: "".to_string(),
            InstrumentationAndMethodsForAstrophysics: "".to_string(),
            SolarAndStellarAstrophysics: "".to_string(),
        }
    }
}
pub struct CondensedMatter {
    pub DisorderedSystemsAndNeuralNetworks: String,
    pub MaterialsScience: String,
    pub MesoscaleAndNanoscalePhysics: String,
    pub OtherCondensedMatte: String,
    pub QuantumGases: String,
    pub SoftCondensedMatter: String,
    pub StatisticalMechanics: String,
    pub StronglyCorrelatedElectrons: String,
    pub Superconductivity: String,
}

impl CondensedMatter {
    pub fn new() -> Self {
        CondensedMatter {
            DisorderedSystemsAndNeuralNetworks: "".to_string(),
            DisorderedSystemsAndNeuralNetworks: "".to_string(),
            MaterialsScience: "".to_string(),
            MesoscaleAndNanoscalePhysics: "".to_string(),
            OtherCondensedMatte: "".to_string(),
            QuantumGases: "".to_string(),
            SoftCondensedMatter: "".to_string(),
            StatisticalMechanics: "".to_string(),
            StronglyCorrelatedElectrons: "".to_string(),
            Superconductivity: "".to_string(),
        }
    }
}
