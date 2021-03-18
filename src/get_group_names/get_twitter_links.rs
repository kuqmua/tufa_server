use std::collections::HashMap;
pub fn get_twitter_links() -> HashMap<&'static str, &'static str> {
    let twitter_sections_links: HashMap<&str, &str> = [
        ("physorg_com", "https://nitter.42l.fr/physorg_com/rss"),
        ("joebarnard", "https://nitter.42l.fr/joebarnard/rss"),
        OK9UWU,
        r2x0t,
        Katha16777,
        PataniLab,
        AlexSerbul,
        TheCherno,
        rustlinz,
        HopeMarsMission,
        dougbinks,
        remodemo,
        italiancpp,
        ilpropheta,
        alepezzato,
        rstropek,
        agrimgupta92,
        GraphicMeetup,
        ShorterLab,
        werf_io,
        Back2Warcraft,
        AvoydGame,
        habr_eng,
        AnalysisSensing,
        abhi_tweeter,
        ozkriff_ru,
        KudSverchkov,
        AstonlabsPurdue,
        kermitmurray,
        OatesLab,
        GeiselBiofilm,
        GamedevStefan,
        ghodges_dev,
        JonathanGMannLD,
        iFiery,
        _jeck,
        krajzeg,
        scottstoll2017,
        WhiteDo27114277,
        vector_of_bool,
        Suthar3Aryan,
        flutter_jobs,
        Jiayin_Cao,
        ScopeShifu,
        ApoorvaJ,
        vexastrae,
        Stefan_W_Hell,
        FlutterWarsaw,
        BocaChicaGal,
        MiSvTh,
        ITSURYUU,
        kushirosea,
        WataruVFX,
        Prokaryota,
        QiaochuYuan,
        ToughSf,
        flight404,
        luwizart,
        angew_chem,
        KnativeProject,
        ulalaunch,
        ChevryMarc,
        guillain_barre,
        AJR_Radiology,
        van__Oijen,
        fasterthanlime,
        mattatz,
        Rocha_Lab,
        zzznah,
        jmutterer,
        Tuatara_VFX,
        Wardl_,
        mobileunderhood,
        doescience,
        raisingsun6665,
        PeterFabianLab,
        BioRender,
        DarArvin,
        podcast_znprod,
        _willfalcon,
        Remesher_,
        ndee85,
        RubenR3D,
        Adrien_nayrat,
        ThePracticalDev,
        polycount,
        kendrickszb,
        ZebrafishRock,
        A_blender_user,
        UnderJS,
        JesseMiettinen,
        KunosStefano,
        RustFest,
        RustConAsia,
        MingshenSun,
        BaiduXlab,
        NatureComms,
        InfographicTony,
        Sanctus_Art,
        jongranskog,
        bagder,
        dimforge,
        carlosedubarret,
        burntsushi5,
        eigensteve,
        dieselframework,
        CliRust,
        lucio_d_franco,
        ShekoHex,
        ShekoHex
        thekbknapp,
        matthiasendler,
        hellorustshow,
        ryan_levick,
        jntrnr,
        vengarioth,
        DPC_22,
        Erstejahre,
        killercup,
        robert_winslow,
        wvo,
        yaahc_,
        Carter_Lab,
        TokamakUI,
        kebabskal,
        Sakura_Rabbiter,
        MichalLytek,
        sagzehn,
        S_LevequeFort,
        The_ArtOfCode,
        awwbees,
        marcel_campen,
        lateasusual_,
        attiegrande,
        pcwalton,
        wittyelk,
        FlutterFireDev,
        kar_sourav,
        Gabriel_Risa,
        BlenderBIM,
        Renato3xl,
        ChenHuang96,
        RaizNewMedia,
        NASAGoddard,
        NASASolarSystem,
        NASAKennedy,
        NASA_Technology,
        nasahqphoto,
        NASA_Astronauts,
        AsteroidWatch,
        medrxivpreprint,
        ReplicabilityG,
        playcanvas,
        AndreaBassi78,
        StereoVinny,
        TidalFlask,
        FengZhuDesign,
        biorxiv_sysbio,
        biorxiv_neursci,
        biorxiv_micrbio,
        biorxiv_genetic,
        biorxiv_evobio,
        biorxiv_ecology,
        biorxivpreprint,
        biorxiv_genomic,
        biorxiv_bioinfo,
        entagma,
        eddbiddulph,
        SoluSerg,
        JamesBlasco,
        BlenderNPR,
        vlusky_husky,
        val_sagrario,
        Abdelfattah__A,
        vprobon,
        joshuamdeguzman,
        flutter_exp,
        letsbuildgg,
        panzerdp,
        _Korchiy_,
        TheGregYang,
        D2L_ai,
        Natzke,
        simonstoschu,
        Underfox3,
        Index_3D,
        benprudhomme1,
        shanenewville,
        brianglancylab,
        Astro_Doug,
        AstroBehnken,
        vr_sebas,
        Leukbaars,
        4DNucleome,
        petervandervel3,
        DunsingValentin,
        stephen_bester,
        scita_lab,
        TheYapLab,
        eriksahailab,
        ActonLab,
        Kaplanyan,
        BCiechanowski,
        OlexaLe,
        JiYiLight,
        cgmastersnet,
        cg_geeks,
        Bbbn192,
        derbender4,
        StashVertex,
        NDunes3D,
        boxvfx,
        karll_henning,
        filedescriptors,
        vcubingx,
        ZanQdo,
        PaoloFurani,
        LabSauer,
        FerrousSystems,
        rust_analyzer,
        rust_gamedev,
        munlangorg,
        AmethystEngine,
        memprotmd,
        lesloew,
        BiophysJ,
        biorxiv_biophys,
        PlantPhys,
        azonenberg,
        johan_elf_,
        kitasenjudesign,
        Jan_de_Vries,
        JXBot,
        bryotweet,
        JacquesLucke,
        NASAPersevere,
        PThuriot,
        AlphaWhiskey82,
        CBI_Pitt,
        YangLiuLab1,
        Geeks3D,
        BrookhavenLab,
        Fermilab,
        alksndrkili,
        Vochsel,
        sai_charan_md,
        WinnichenkoD,
        LesleyLai6,
        VGVentures,
        SuprDeclarative,
        cgonfire,
        luigifcruz,
        agriclaudia,
        keyframes_tw,
        nikomatsakis,
        ServoDev,
        rustwasm,
        read_rust,
        rustaceanfm,
        jonhoo,
        4minus1d,
        CorneliusGati,
        NucleusSciTalks,
        QianPeterSu,
        phantom_owl,
        serhii_rieznik,
        FritzscheLab,
        abbelight,
        GoncaloFDS,
        RandomBlendings,
        KloudStrife,
        dchaplot,
        oxcsml,
        jinxu06,
        kasparmartens,
        emidup,
        MarcusHilsdorf,
        martin_gorner,
        arXiv_Daily,
        a13xp0p0v,
        rajammanabrolu,
        SeptinLab,
        daniela_barilla,
        sqaunderhood,
        TychoBolt,
        SirWadeFX,
        SebDeguy,
        natBME,
        pavmike,
        im_galad,
        ZoeyFan723,
        ShechtmanLab,
        eric_heitz,
        torch_in_sky,
        fpelisch,
        ritafior,
        agvanimation,
        kD3AN,
        guiwitz,
        miguelbandera,
        PappulabWashU,
        GregMadison,
        ebarranko,
        danielsantalla,
        fayezsalka,
        ilya_aparin,
        NaturallyCG,
        OrestesGaolin,
        erindale_xyz,
        chippwalters,
        Raspberry_Pi,
        ykilcher,
        josmil1,
        rileyb3d,
        asahidari,
        revoider,
        blacksquirrel__,
        GuillaumeLample,
        belle2collab,
        basit_ayantunde,
        dwrensha,
        Andi_Microscopy,
        RupeshMandke,
        rustbeltrust,
        RustVideos,
        rustconf,
        newrustacean,
        ThisWeekInRust,
        zombodb,
        svartalf,
        tokio_rs,
        RustTrending,
        RustLibHunt,
        haraldhoyer,
        246R_Bloomin,
        timsneath,
        J_A_C_S,
        BettiniGabe,
        IlyaKuzovkin,
        pgexperts,
        2ndQuad,
        PostgresWeekly,
        Cuboxel,
        maxSigma_,
        ArtFromRachel,
        RanaHanocka,
        yoongs,
        CrossmindStudio,
        danielepolencic,
        phosphoer,
        jobtalle,
        wiersma_ruben,
        SilenceMoon_Yue,
        linolafett,
        Froyok,
        exppad,
        IndiaFlutter,
        DevenJoshi7,
        erikskog_,
        Deep__AI,
        EuropHospital,
        rustlang,
        JoshWComeau,
        deno_land,
        Icare3D,
        tunabrain,
        Stubbesaurus,
        tristanbrindle,
        NiloCat_Colin,
        Oscurart,
        Atrix256,
        nixcraft,
        Mirko_Salm,
        FlutterLDN,
        FlutterReleases,
        geekmz,
        CGuivant,
        felangelov,
        artofjeffp,
        brendandjcad,
        Vasyl72728301,
        simurai,
        clayxels,
        VisualStudio,
        imthepk,
        phi_lira,
        MiegakureGame,
        PaulC04,
        boksajak,
        stevestreeting,
        joachimgoedhart,
        Cabbibo,
        london_lab,
        games_inu,
        m_schuetz,
        PhysicalAddons,
        larry73451236,
        portnov9,
        AnatoleDuprat,
        MartinStich,
        0xca0a,
        rianflo,
        John_O_Really,
        arecaplm,
        crascit,
        CERN,
        neilt3d,
        sylefeb,
        _TomekS,
        BartWronsk,
        TheAllenChou,
        3drwny,
        suishess,
        xbresson,
        harrietm11,
        biggsneal1,
        SpectreSkully,
        matthen2,
        fael097,
        MacchiatoLycan,
        juulcat,
        artistcdmj,
        mikulasflorek,
        colejefferies,
        nelstuff,
        chriskwallis,
        ispc_updates,
        noazark,
        SenguptaUmd,
        KevinCadieuxMS,
        ejdeon,
        HCI_Research,
        kenshirriff,
        mraleph,
        JS_Cheerleader,
        adampi,
        LaineBioImaging,
        notargs,
        DavideCalebiro,
        BioGridGame,
        stelabouras,
        hekiba_io,
        MasterWhet,
        vitos1k,
        BrianLeleux,
        FAGIOLOVOLANTE,
        keenanisalive,
        victorvdr9,
        jeffamstutz,
        quasimondo,
        GoogleAI,
        AravSrinivas,
        uschmidt83,
        jaguring1,
        louisdumont,
        happy_modeling,
        AliceInNanoland,
        WWeynants,
        godsgreg,
        dedouze_,
        nik_vili,
        AlainChedotal,
        var_bincom,
        romainguy,
        MattNiessner,
        ewers_helge,
        PixelClearr,
        Inasa_Fujio,
        PlatypusAdvent1,
        fredric_,
        luamono,
        VicDoval,
        ProdeusGame,
        llazyadM,
        Carlillo,
        Donzanoid,
        pixnblox,
        stefanzellmann,
        Wayward_Art_Co,
        mluparu,
        johnparsaie,
        panlepan,
        dfranx_,
        flutterinst,
        flutterdevs,
        FlutterWk,
        lipmanya,
        NASA_SLS,
        openspim,
        acmarr,
        KSpaceAcademy,
        PromPreprint,
        AurelioReis,
        neilbickford,
        MaxPuliero,
        hollasch,
        CodeerDev,
        FakhriLab,
        chrisprenn,
        __xuorig__,
        SongForHumanity,
        erikswahn,
        DannyBittman,
        schneckerstein,
        Boycraf19492179,
        javarevisited,
        Azadux,
        GradManuel,
        TylerG1998,
        hsalis,
        inkasipola,
        syoyo,
        kotsoft,
        ISS_Research,
        compoundchem,
        gp_pulipaka,
        sulco,
        martin_maas,
        ourmachinery,
        jrpowers,
        kyXtak,
        ProfTomEllis,
        C_M_Hobson,
        flutterize,
        UriGoldshtein,
        FullstackDevJS,
        NECKOPN,
        Feyris77,
        norbertkozsir,
        jhorikawa_err,
        CodyWinch,
        ChicagoFlutter,
        FlutterEurope,
        _eseidel,
        redbrogdon,
        filiphracek,
        soragnilab,
        jlizier,
        zipcpu,
        NOTLonely92,
        UE4Memes,
        Nosferalatu,
        djowel,
        PeterWBattaglia,
        BaranLabReads,
        moxstudios,
        Robert12415877,
        davephoffman,
        v6_prime,
        tvaneerd,
        JesseBrophy,
        jerbotnet,
        3blue1brown,
        Astro_Jessica,
        johannaivaska,
        anna_medyukhina,
        jonathangrimm,
        aras_p,
        MagnusL3D,
        BlenderDiplom,
        cinedatabase,
        rita_strack,
        OlixOliver,
        hey_michaelh,
        kevin_tsia,
        gamozolabs,
        pixelmager,
        JanWillemTulp,
        ddiakopoulos,
        SasaBudimir,
        medical_xpress,
        yaroslav_ganin,
        charlietcnash,
        cobalt_kura,
        Koola_UE4,
        SignalCambridge,
        moyix,
        mpicbg,
        diffusiveblob,
        ankurhandos,
        HohlbeinLab,
        FredTingaudDev,
        NaturePhysics,
        physorg_com,
        WendeNGibbs,
        ambrosiussen_p,
        CoolSpotDreamer,
        maartenjhof,
        ma1andrea,
        anatudor,
        MartinNebelong,
        Addgene,
        DennysKuhnert,
        wilnyl,
        alexis_gil,
        FaustXins,
        simon_stix,
        erturklab,
        jaaanaru,
        flutter_school,
        AndrePulschen,
        BrianKelch,
        alexanderbelyy1,
        myosinactncrazy,
        benoitbruneau,
        HubbleTelescope,
        EvanHallMD,
        sebmck,
        DrSalazarMejia,
        FabioSchutz78,
        DrNikhilJoshiMD,
        DrOmarMian,
        ShaanDudani,
        neerajaiims,
        QueckOlli,
        MatNiedoba,
        ferristweetsnow,
        FlutterBlogs,
        freeman_as,
        pepijndevos,
        horse_js,
        MyNameIsMJP,
        ScienceTM,
        elmanmansimov,
        inresin,
        __simt__,
        3Dmattias,
        NatRevImmunol,
        NatRevClinOnco,
        BennyGovaerts,
        takase_s,
        kytwb,
        jsoriamd,
        GowthamRaj100,
        katsuya_tsukui,
        Booster_Buddies,
        AndroidDev,
        IBMDeveloper,
        Dr_RaviMadan,
        preshing,
        zaidalyafeai,
        kimitalvitie,
        ShinpeiKato,
        11thDream_Game,
        kozzzlove,
        shaderpunk_jp,
        sedrewed,
        ico_TC,
        slurpsmadrips,
        biz84,
        SebastianLague,
        flutterdevcast,
        codemagicio,
        d_gfx,
        DartHype,
        DartCode,
        FlutterComm,
        Flutter_Flakes,
        flutteriodaily,
        dart_lang,
        ShuregDenisov,
        mrdoob,
        FerrenceG,
        AssemblyScript,
        torch2424,
        FirefoxDevTools,
        mhadaily,
        amuellerml,
        _ramoreira,
        IntelSecurity,
        scott_e_reed,
        marsrader,
        Smerity,
        three_cube,
        FullStackFest,
        soshace_com,
        AlinaShumarina,
        strewnify,
        NASARoman,
        Aidan_Wolf,
        spornslab,
        GregScott_photo,
        deanwampler,
        TimSweeneyEpic,
        Inoryy,
        mike1pol,
        tobiasmarciszko,
        greggman,
        myshov,
        LuciusFekonja,
        miloseviclab,
        MadelonMaurice,
        AndreySozykin,
        MatthewDean3D,
        cman2k,
        Dr_Ivanoncologo,
        CNC_Kitchen,
        arcanis,
        rezoner,
        slashML,
        TaroOzaki,
        seanbax,
        Firebase,
        _davideast,
        BP_Hutch,
        a00rs,
        MarescaLab,
        NetanelBasal,
        wysscoray,
        nori_shinoda,
        ChemPlusChem,
        strudlzrout,
        glukozica,
        fraser_lab,
        marian42_,
        MarcelAMller,
        KLEIJ_ICIQ,
        ECycles1,
        MolinaGroup,
        Cyanilux,
        nestframework,
        N_Tepluhina,
        eems_mit,
        sitnikcode,
        florianjug,
        benawad,
        EmporiumThought,
        ClassyDogFilms,
        sculptjanuary,
        lunasorcery,
        mattstark256,
        ICCV19,
        GitNationOrg,
        mclow,
        LeiTian14,
        GolangKazan,
        AterCattus,
        behindthescope_,
        Johannes_Karges,
        ntziachristos,
        rsalakhu,
        rauchg,
        vercel,
        FrederickWelt,
        leondenise,
        nizzle_fe_shizz,
        NVIDIAEmbedded,
        NVIDIAHPCDev,
        NVIDIADC,
        NVIDIAGameDev,
        david_obrien,
        KKyrimis,
        US_SpaceCom,
        JimBridenstine,
        keen_tools,
        Aiims1742,
        salkinstitute,
        MrB_Jensen,
        ProgrammerLin,
        TheHackersNews,
        zetuZT,
        denisivanov,
        MicroArchConf,
        sigarch,
        TitusWinters,
        harrism,
        LambdaConcept,
        SAFARI_ETH_CMU,
        _onurmutlu_,
        cppedinburgh,
        nicoscherf,
        Astro_Ferg,
        massimorighi,
        prince_xml,
        KAvignon,
        NEUBIAS_COST,
        UCL_IPLS,
        JPMajor,
        _LaszloZoltan_,
        Simon_Houdini,
        ProjectJupyter,
        fchollet,
        DrCEWilloughby,
        InstLatX64,
        dnesteruk,
        dendibakh,
        csssr_dev,
        ThompsonLab,
        JCellBiol,
        Dev_Bio_Journal,
        DevelopmentalDy,
        bxv_dev,
        aslushnikov,
        MattMirrorFish,
        3Lateral,
        thinkmariya,
        Sam_Makes_Games,
        mattmiesnieks,
        g33konaut,
        murosyoukei,
        ASCBiology,
        cshperspectives,
        detective_horse,
        robinmanuelt,
        jgsogo,
        careyjans,
        NicoJosuttis,
        Icy_BioImaging,
        BioImagingUK,
        roshan_m_rao,
        bgolus,
        stoyanstefanov,
        brian_lovin,
        rochaandreal,
        LaissuePhilippe,
        NikonSmallWorld,
        JustinPaver,
        RReverser.
        wasmerio,
        GeneralistP,
        2112_games,
        WindowsDocs,
        lucengame,
        ZaedDB,
        RebeccaRHelm,
        NathanShaner,
        FiolkaLab,
        RetoPaul,
        KerenGu,
        GoodAIdev,
        DanfengCai,
        andrey_akinshin,
        the_f_key,
        _BD3D,
        forwebdev,
        jeffbarr,
        GMFHx,
        d_brueckner,
        DougPShepherd,
        jengreitz,
        AcceleratorNick,
        MaciejTreder,
        damiandn,
        CharlotteFare,
        BlackHC,
        DesignSpark_JP,
        DonaldM38768041,
        aire_team,
        BateupLab,
        alt_kia,
        marcan42,
        luizkruel,
        obenjiro,
        lradoshevich26,
        SedonaMurphy,
        vanderlin,
        ovrweb,
        TrackingActions,
        KshitizKz,
        posva,
        EmmanuelMagnier,
        KyongFAM,
        srush_nlp,
        mlfrg,
        ManuelaXibanya,
        mattersOfLight,
        AlexKontorovich,
        LawdOdin,
        skx_doom,
        trusty_games,
        mxsage,
        grpcio,
        Tankred_Daeron,
        pitercss_meetup,
        JasperRLZ,
        longbool,
        BartekMoniewski,
        3d_eric,
        rage_monk,
        bunopus_en,
        AJ_FI,
        amel_true,
        mohitban47,
        jaseweston,
        parlai_parley,
        ensou_art,
        VitaliyKirenkov,
        xenophar,
        ppaawweeuu,
        grgrdvrt,
        FranziskaPanter,
        Blackrabbit99,
        kottans_org,
        xanf_ua,
        miziziziz,
        shwars,
        NathanGDQuest,
        godotengine,
        _AmazingAssets,
        trav_downs,
        pierrci,
        _alexeykalinin,
        lightarchitect_,
        evilmartians_ru,
        SaiyamPathak,
        vitessio,
        PostCSS,
        kotlin,
        archaeal,
        _KudoHiroyuki,
        jaredpalmer,
        _benleblanc,
        johntwolives,
        biorxiv_cellbio,
        Vitalliumm,
        hamiltonulmer,
        LevelPixelLevel,
        38912_DIGITAL,
        remi_creative,
        arborillustrate,
        profbof,
        h3r2tic,
        polygonrunway,
        izutionix,
        _poei,
        quixeltools,
        argyleink,
        123gas321,
        pky_blog,
        joaomoreno,
        james_madhacks,
        KaverinaLab,
        LucaRood,
        yarpoplar,
        peterekepeter,
        HolyJSconf,
        alfcnz,
        memecrashes,
        MarcoDiVita6,
        TF_siri,
        NateMorrical,
        NanoLiveLtd,
        ____lighthouse,
        aerotwist,
        mathias,
        jaffathecake,
        DasSurma,
        LightUpScience,
        Sentdex,
        DotNextConf,
        nevalau,
        CEITEC_Brno,
        ozmant,
        nathanduck88,
        mikegarndesign,
        johannes_wilde,
        The_BenMears,
        museumofcomm,
        AboticsG,
        orestiskon,
        KochkinGames,
        kengoito1110,
        RubenEVillegas,
        siddharthkp,
        Pawige,
        t_looman,
        DiegoSaraiva,
        mjhigley,
        profLiangGao,
        GraceIHsu,
        HansClevers,
        neurosocialself,
        kammysliwiec,
        QueerJS,
        hardsci,
        twi_mar,
        PolystreamE,
        ChengXianrui,
        _jdevos_,
        NikolayOskolkov,
        abeysaurus,
        skaven_,
        babylonjs,
        ThomasKoleTA,
        maisam_hosaini,
        BayuItra,
        fletchgraham,
        JayMBroderick,
        JoeyDeVriez,
        70_cg_art,
        devoopsconf,
        rickyreusser,
        reduzio,
        graphqlsummit,
        BigDataToolsJB,
        AndrewGYork,
        lopoisaac,
        anuraghazru,
        DmitryMakhnev,
        Thom_Wolf,
        khtrammell,
        amaarora,
        ak92501,
        Dogen,
        jcjohnss,
        dgaboriau,
        ClangPowerTools,
        HungryProton,
        davorjord,
        jfranmatheu,
        andrei_faraon,
        css_moscow,
        AvitoTech,
        P5_keita,
        rspadillam,
        valentin_galea,
        ferrisjabr,
        loloarmdz,
        JuliusHorsthuis,
        AlexShcheglovit,
        BoneStudioAnim,
        Ulrike_Boehm,
        Rich_Harris,
        WilliamChyr,
        haesleinhuepf,
        MitoPsychoBio,
        GridDynamics,
        matvi3nko,
        guycalledfrank,
        OpenVX,
        posipov,
        robertghrist,
        IgorKhalins,
        USCLONI,
        johncarlosbaez,
        bluefox_3d,
        DesignMuseum,
        GSG3D,
        csswizardry,
        DeevanHoven,
        DevSchacht,
        VicidominiLab,
        harryh___h,
        PetrukStanislav,
        ajitjohnson_n,
        miiiiiki14,
        phioptics,
        qlilab,
        marinintim,
        vuejs,
        sveltejs,
        PaulMaly,
        BradeskoUrban,
        ProcessingOrg,
        tadatuta,
        PASSO1031,
        dsgoodsell,
        logux_io,
        PixelsCommander,
        georgeMandis,
        evilmartians,
        wegorelax,
        adriancolyer,
        dark_mefody,
        r_FlutterDev,
        DataScienceCtrl,
        lucasmpinto,
        jmztn,
        VisrealmGames,
        MetastasisRS,
        tintwotin,
        optrickster,
        McNopper,
        notrab,
        harker451,
        AndrewM_Webb,
        _Yadoob,
        Golgi_Willikers,
        RaphaelRau,
        webmaxru,
        denar90_,
        fatehmtd,
        OpticalSociety,
        OSAPublishing,
        DeepLabCut,
        mueller_physics,
        webkit,
        jjding99,
        jackclarkSF,
        Puasonych,
        The_Whole_Daisy,
        BlenderGuruOOC,
        mast4461,
        jonkantner,
        F_Rimasson,
        CShorten30,
        Sketchfab,
        DommiGraf,
        iliyang,
        RichardSocher,
        huggingface,
        balov_bohdan,
        tardigradopedia,
        Nothke,
        HakanssonAugust,
        ml_review,
        beniroquai,
        the_glenster,
        code_report,
        bencbartlett,
        codingapegames,
        snehaark,
        Claudio_Tulio,
        habr_popsci,
        habr_com,
        seb_ruder,
        raroni86,
        AnalyticMike,
        kimmokaunela,
        ChromeDevTools,
        marquetand,
        PiterPy,
        pragmaticml,
        lisyarus,
        OpenBehavior,
        BonnetSylvestre,
        physicsJ,
        dsuoch,
        obiltschnig,
        Wappalyzer,
        sakanaya,
        _baku89,
        cgbassa,
        AdamRackis,
        andreyknvl,
        Inamura_JIN,
        erasmus_brosdau,
        planetpostgres,
        a_saito,
        ForrestTheWoods,
        ephtracy,
        maxonjapan,
        HuntingFluff,
        algomystic,
        EsIstBenjamin,
        Bookyakuno,
        mradfo21,
        mogumoguTaupe,
        dailydotdev,
        yusuketokuyoshi,
        yurzui,
        2ality,
        PawelDziepak,
        grigoryvp,
        CGGeeks,
        zazamorga,
        domipheus,
        LostTrainDude,
        Syziph,
        Bachadam4,
        lileaLab,
        fujii_masao,
        asta18425,
        NatMetabolism,
        BuschmannHenrik,
        tissue_engineer,
        DDNewsOnline,
        ryxcommar,
        DeformDynamics,
        AlexStrook,
        BcubedLabs,
        matkovskid,
        Axksel,
        karlis_stigis,
        Nat_Prunet,
        arc4g,
        manuel_donofrio,
        FeydyJean,
        cjliu10,
        supernlpblog,
        tomaheckel,
        pelengami,
        DesolusDev,
        julian_moger,
        ej_sa,
        jeffandcasey,
        CouturierArnaud,
        stephen_wolfram,
        erkaman2,
        frontakk,
        udayvunnam_,
        yosikage0911,
        themikepan,
        RubyInside,
        rails,
        CodeceptJS,
        Nahuel_Belich,
        threejs_org,
        connrbell,
        Severin_b3d,
        LabRadenovic,
        manorlaboratory,
        worldsday,
        DailyMOBAHero,
        ShinjiOgaki,
        GameTxtures,
        DirectoryRanger,
        krimzky,
        gatecatte,
        oe1cxw,
        Dilmerv,
        guijacquemet,
        AndrewJDR,
        _kzr,
        klemen_lozar,
        etopirika5,
        Angular_InDepth,
        maxkoretskyi,
        Nikhil15,
        embojournal,
        AlexM_Tweet,
        photopeacom,
        jcant0n,
        krddevdays,
        AnnaMimik,
        zkajdan,
        richturn_ms,
        DirectX12,
        linclark,
        rick_viscomi,
        nanomicroscopy,
        FoxChaseCancer,
        RomanPapush,
        tuxedolabs,
        siraniks,
        ribponce,
        HarryAlisavakis,
        proxeIO,
        oh_that_hat,
        UberEng,
        MesoSpim,
        panoskarabelas1,
        alex_conneau,
        ReactTraining,
        ShorterLabGroup,
        basvasilich,
        jsunderhood,
        AffectiveCpp,
        thestr4ng3r,
        george_superres,
        ShaderGuy,
        BenKrasnow,
        plugins4d,
        philwalton,
        frankiezafe,
        PDBeurope,
        PDBimagebot,
        dcpage3,
        deleteman123,
        JoeHughesDev,
        sftrabbit,
        appleseedhq,
        brain_graft,
        andreintg,
        archyufa,
        timur_audio,
        anastasiaopara,
        hesse_marina,
        slinnarsson,
        tommulgrew,
        seandavis12,
        matamalaortiz,
        ThinMatrix,
        Mayeul_Collot,
        Lab_Klymchenko,
        aklymchenko,
        cpponsea,
        jytinevez,
        DOOMReboot,
        WrocTypeScript,
        Bananaft,
        pyromuffin,
        cgpov,
        quantumjot,
        wanimation2910,
        FritzPreusser,
        JAMA_current,
        NDC_Conferences,
        corednsio,
        Cyberarms,
        k_kondrak,
        ibartomeus,
        garethr,
        kazu_maeshima,
        bigdata,
        NASASun,
        patriciogv,
        golangweekly,
        z3r0trust,
        AlbertoCGArt,
        adam_k_glaser,
        Lub_Blub,
        FuriKuri,
        JonTheGeek,
        FlyGutLab,
        bamimages,
        alexellisuk,
        bengoult,
        AsmundSchei,
        marouanerassili,
        MichalZiulek,
        ToshiCG,
        DenisNovikov25,
        hintbits,
        dataegret,
        botchagalupe,
        PeterZaitsev,
        coreos,
        pchico83,
        dvyukov,
        valgog,
        golang,
        demishassabis,
        devopsdeflope,
        tsafin,
        bashmish,
        mitchellh,
        OmniTI,
        _olau_,
        advinst,
        pragma37,
        nidhi_vijayan,
        nazzagnl,
        MadeByMarcel,
        _wusher,
        IngoWald,
        timneutkens,
        StudioTatsu,
        KMkota0,
        alessandrozompa,
        BrutPitt,
        46frsts_dreams,
        s0lly,
        _xmasjazzy_,
        xamarinhq,
        natfriedman,
        bashrw,
        DenisKrndija,
        KostasAAA,
        HHMINEWS,
        WilsonAdams_,
        mewlist,
        DavidKPiano,
        martweig,
        Domenico_brz,
        leanpub,
        _SergVasiliev_,
        ZivShulman,
        PhysRevA,
        PhysRevMater,
        PhysRevC,
        PhysRevD,
        freeCodeCamp,
        CodePen,
        KrasimirTsonev,
        arxiv_cs_cl,
        ai_papers,
        arxiv_cshc,
        arxiv_csgr,
        arxiv_cscv,
        arxiv_cscl,
        MRC_LMCB,
        angular,
        AR_Ojisan,
        scottjehl,
        EdgeDevTools,
        joeltelling,
        OptNanoscopy,
        Themaister,
        l_masu,
        ElisaDEste,
        BalzarottiFran,
        theCGchannel,
        tomB_saikaya,
        GDGish,
        dotnet,
        biophotonicat,
        PaulChambers3D,
        JoseFalko,
        handmade_hero,
        wellecks,
        NovickGreenplum,
        mitsuhiko,
        mdhowellmd,
        jbaruch,
        CiroContns,
        Mrdodobird,
        thegalshir,
        abigail_e_see,
        gekido,
        TangentVector,
        AnimationMentor,
        createthematrix,
        hotchipsorg,
        DirkBrockmann,
        NICOatNU,
        ChemCatChem,
        green_rsc,
        ACSCatalysis,
        NatureCatalysis,
        cssliveru,
        htmlacademy_ru,
        fernandojsg,
        bgoncalves,
        lonelyvertex,
        ARRS_Radiology,
        DeepSpaceBanana,
        drkjjeffery,
        NVIDIAAIDev,
        compositionalit,
        PostgresSupport,
        frontendmentor,
        FlipTanedo,
        attila_afra,
        RexDouglass,
        antonschrein,
        refikanadol,
        kentbarber,
        BinaryCoder,
        Olmirad,
        phi6,
        Clotmaster,
        pvs_studio,
        ziyangwen,
        AssetLove,
        ymt3d,
        quantumpenguin,
        ushiostarfish,
        baldurk,
        SaschaWillems2,
        FlohOfWoe,
        jbh3d,
        JordanBestwick1,
        DottiDeveloper,
        CaoMoji416,
        nice_byte,
        virakri,
        jonathanfly,
        ScienceMagazine,
        baldand,
        CeMoCreates,
        imachinarium,
        kanetaaaaa,
        Keegan_Keene,
        th127,
        vfxplatform,
        YuriKovelenov,
        BenMauro993,
        LiamLepinay,
        nmap,
        raesene,
        csa_asc,
        scionti_design,
        kentcdodds,
        Weaver_Dev,
        nojustesen,
        aiekick,
        gdm3000,
        thockin,
        tdhooper,
        CloudNativeFdn,
        EnvoyProxy,
        christianposta,
        sforkmann,
        RocketRundown,
        nikodemmalec93,
        EfflamMercier,
        anim_Matt,
        zakura44219277,
        JonasTyroller,
        3D_director,
        SaraSoueidan,
        AntonioGil_Jogo,
        eijiphotogra,
        Daajust,
        diharaw94,
        HighlySpammable,
        unrealcg,
        minionsart,
        JulienKaspar,
        mano_wii,
        BlenderConf,
        pokedstudiouk,
        YegorSmirnov_,
        mariotelfig,
        Trinumedia,
        _naam,
        Lilium,
        MjTheHunter,
        AlexFellows1,
        _staggart_,
        WITHIN,
        romainderrien,
        andrewgwils,
        BakalLabICR,
        FreyaHolmer,
        cubitstudios,
        nolar,
        OracleDatabase,
        ArtStationHQ,
        GalacCuriosity,
        JustinMSolomon,
        mihail_eric,
        lightdelay,
        StaggeredFight,
        addyosmani,
        Code_Analysis,
        CasualEffects,
        psjarlo,
        jeremyhu2016,
        AirbusSpace,
        vishnuganti,
        henningsanden,
        InertialObservr,
        FewesW,
        JonasDichelle,
        s3r10s1n,
        satcy,
        MAG2ART,
        KangarooPhysics,
        kaggle,
        odsai_en,
        runwayml,
        territorystudio,
        Maurice_Y_Lee,
        TEDD_IO,
        richnosworthy,
        CuPy_Team,
        PreferredNet,
        UX3DGpuSoftware,
        rmartinledo,
        MStrehovsky,
        llennoco22,
        noribros,
        NVIDIAGeForce,
        stefan_3d,
        keenanwoodall,
        joe_ihdt,
        qoolheart,
        wackyvorlon,
        ryotom761_cc2,
        arturonereu,
        NVIDIADesign,
        OneDevloperArmy,
        prvncher,
        BrunoLevy01,
        satishgoda,
        cayou66,
        notCamelCase,
        Daily_ML_Tips,
        ryanjfleury,
        0xgalz,
        agni_flare,
        FinnGrayHiggins,
        Trino_dev,
        arggaurav,
        HutchingsINovae,
        gopiterjs,
        ArtellBlender,
        FlippedNormals,
        iquilezles,
        Blender_Cloud,
        derrkdotcom,
        BlenderToday,
        csharpfritz,
        marcinignac,
        nmwsharp,
        GPUOpen,
        non_manifold,
        fgrassard,
        maztheegg,
        Adsk3dsMax,
        AdskMaya,
        dsymetweets,
        shiropen2,
        bkaradzic,
        MGDev91,
        PiotrPadlewski,
        joost_v_amersf,
        arkosiorek,
        unity3dexpert,
        AssetStore_JP,
        smashingmag,
        faaaaarck,
        thegraphqlconf,
        schickling,
        oauth_2,
        zoox,
        mxstbr,
        j_bertolotti,
        ylecun,
        ArnaudDevelay,
        phoronix,
        PaulDreik,
        CiscoDevNet,
        0by0ne,
        JangaFX,
        DanMillerDev,
        openvisualfx,
        UnityBerserkers,
        david_cobac,
        EMDB_EMPIAR,
        hdmacgillavry,
        rich_lord,
        wanoco4D,
        NirantK,
        mattgodbolt,
        supahvee1234,
        robwirving,
        llvmorg,
        pocoproject,
        conan_io,
        dascandy42,
        Guriwesu,
        ben_deane,
        CppLibHunt,
        rainer_grimm,
        lasote,
        antohabikineev,
        bincrafters,
        solvingj,
        ValdikSS,
        CompilerCpp,
        SSE4,
        ICSEconf,
        filonovpv,
        afiskon,
        LouisDionne,
        ericniebler,
        AlisdairMered,
        stdcppru,
        habr_cpp,
        cppjedi,
        blelbach,
        cpp_rocks,
        Scott__Meyers,
        I_Sadchenko,
        NMakarova,
        EventSpaceBY,
        anastasiak2512,
        CppSage,
        llvmweekly,
        GorNishanov,
        chandlerc1024,
        a_williams,
        clion_ide,
        cpp_russia,
        ivan_cukic,
        CppLondon,
        zamazan4ik,
        shiffman,
        thomashpark,
        apollographql,
        helferjs,
        chiu_hans,
        quocleix,
        youyuxi,
        jetbrains,
        MaterialUI,
        FlutterDev,
        brian_d_vaughn,
        npmjs,
        typescript,
        reactjs,
        ctnzr,
        PythonWeekly,
        SciPyTip,
        ThePSF,
        PythonStack,
        nodkz,
        OskSta,
        HenriquesLab,
        tyler_j_pearson,
        alex_silver,
        ReactSummit,
        nparashuram,
        AlanZucconi,
        nibillii,
        mldcmu,
        MarkLovesTech,
        fbOpenSource,
        Gaxil,
        ahachete,
        TooMuchVoltage,
        KidMograph,
        tejasdkulkarni,
        szeloof,
        GoogleOSS,
        hissearnts,
        NeurosurgeryCNS,
        TheJNS,
        unity3d,
        UnrealEngine,
        AdiShavit,
        KielFiggins,
        TempleDoorGames,
        5agado,
        AFX_LAB,
        agarwl_,
        andyduboc,
        LeaVerouDev,
        arsiliath,
        oddviz,
        nicolas_ceriani,
        _jura_z_,
        m_ninepoints,
        clattner_llvm,
        jimmygunawanapp,
        BachFrancis,
        Aaronmeng5896,
        boubek,
        CppConferences,
        CompileExplore,
        paulclementstv,
        jendrikillner,
        zalbard,
        thecgwiki,
        SmartBiology3D,
        computingnature,
        TritzChristophe,
        postgresql_007,
        venkmurthy,
        blendermarket,
        PoliigonHQ,
        abyrd89,
        ZacSwider,
        OmarEmaraDev,
        cppchat,
        IdentyTee,
        ReactCosmos,
        Andreas__Fertig,
        ChemicalScience,
        InorgChem,
        ChemistryNews,
        NatRevMater,
        NatureNano,
        NatureChemistry,
        NatureMaterials,
        binzmit,
        laanlabs,
        SindySaalfeld,
        oenvoyage,
        Txx_CG,
        kame404,
        FAB_365,
        garett,
        MDA_space,
        thibaut_brunet,
        DukeGunston,
        Yo_Brilly,
        Disting,
        Merkvilson,
        CAPCOM_VFX,
        kenpex,
        gorosart,
        PatriceRoy1,
        BigelowSpace,
        miniciv,
        Yokohara_h,
        ShoelessPeacock,
        duhroach,
        glaucolonghi,
        cgcookie,
        AIBlender,
        blendswap,
        BlenderArtists,
        karpathy,
        BaiduResearch,
        deeplearningldn,
        DeepLearningHub,
        StatMLPapers,
        TheDuckCow,
        GoetzJacky,
        maxoomen,
        greje656,
        MatRopert,
        pacificplusplus,
        JoBoccara,
        JavaScriptKicks,
        G_Revisited,
        bitdev_,
        fenbf,
        TermiCmd,
        flex_ferrum,
        cjones3D,
        mrkkrj,
        NVBackchannel,
        LostInSteve,
        safouen_artist,
        FlipFluids,
        TCox_Lab,
        thomasrcox,
        hardmaru,
        sebirit,
        ArvidGerstmann,
        Nachoconesa1,
        IMilk_ManI,
        DmitryUlyanovML,
        stanfordnlp,
        facebookai,
        TubeTimeUS,
        lotsalote,
        IdoAmitLab,
        xoofx,
        K8Spin,
        MarcoPPavanello,
        AaronCovrett,
        radekosmulski,
        sytses,
        Data_AI_Summit,
        AbnerCoimbre,
        CellReports,
        broadinstitute,
        NatureGenet,
        OaklandUniv_CBR,
        curtisjamesholt,
        dan_abramov,
        PRybitskyi,
        Bamanador,
        arthur3486,
        fvsch,
        SoerbGames,
        NaveenS16,
        MalmoCpp,
        djdavidsonlab,
        NASAWebb,
        NASAUniverse,
        NASAFermi,
        chandraxray,
        NatureAstronomy,
        andrew_gresyk,
        TyskaLabActual,
        Astronut099,
        RavenKwok,
        ACCUConf,
        zeuxcg,
        antonio_vfx,
        aureliengeron,
        MGreenLightning,
        ari_rubin,
        MacTuitui,
        andy_pavlo,
        inversed_ru,
        CTRLlabsCo,
        progfix,
        wileyneurosci,
        NatureNeuro,
        NatRevNeurosci,
        NeuroCellPress,
        neurochallenged,
        sofia_deleniv,
        NathanRoberts17,
        12minutesgame,
        shafikyaghmour,
        ciura_victor,
        SuperluminalSft,
        OpenAPS,
        github,
        getbootstrap,
        sublimehq,
        fatlimey,
        inversepixel,
        handmade_net,
        LJ_1102,
        w23ru,
        twominutepapers,
        SebAaltonen,
        gabrielpeyre,
        pervognsen,
        self_shadow,
        pointinpolygon,
        SebHillaire,
        EricLengyel,
        nickreal03,
        BostonDynamics,
        MgmsUpdates,
        sinusoidalen,
        joelvaneenwyk,
        RudyMichau,
        the_mantissa,
        bep_janssen,
        GregZaal,
        Ethan_Snell,
        wbmoss2,
        LukeLambs,
        SwedenCpp,
        avast_cppmeetup,
        











    ]
    .iter()
    .cloned()
    .collect();
    twitter_sections_links
}
