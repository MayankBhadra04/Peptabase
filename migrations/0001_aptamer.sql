CREATE TABLE aptamers (
    aptamer varchar(39) DEFAULT NULL,
    target varchar(67) DEFAULT NULL,
    apt_type varchar(10) DEFAULT NULL,
    length varchar(3) DEFAULT NULL,
    sequence varchar(42) DEFAULT NULL,
    effect varchar(285) DEFAULT NULL,
    reference varchar(415) DEFAULT NULL
);
CREATE TABLE pending_list (
    id SERIAL PRIMARY KEY,
    email varchar(50) NOT NULL,
    aptamer varchar(39) DEFAULT NULL,
    target varchar(67) DEFAULT NULL,
    apt_type varchar(10) DEFAULT NULL,
    length varchar(3) DEFAULT NULL,
    sequence varchar(42) DEFAULT NULL,
    effect varchar(285) DEFAULT NULL,
    reference varchar(415) DEFAULT NULL,
    status varchar(10) DEFAULT 'Pending'
);
CREATE TABLE users (
    email VARCHAR PRIMARY KEY,
    password VARCHAR,
    admin boolean
);

INSERT INTO aptamers (
    aptamer, target, apt_type, length, sequence,
    effect, reference
)
VALUES
    (
        'TRIAP alpha', 'inhibit  Rho GEF Trio and its oncogenic isoform Tgat',
        'Anticancer', '42', 'AREGADGAICGYNLATLVMLGPSERVFCPLCEPCSSDIYELM',
        'inhinit oncogenic RhoGEF', 'aptamer-Derived Peptide Inhibitors of Rho Guanine Nucleotide Exchange Factors Susanne Schmidt1 , Anne Debant'
    ),
    (
        'TRIAP BETA', ' TRIO GEF2(cDNA)',
        'Anticancer', '20', 'IPKKYERSSPLRRPVRMMLR',
        'inhibit oncogenic rhogef', 'aptamer-Derived Peptide Inhibitors of Rho Guanine Nucleotide Exchange Factors Susanne Schmidt1 , Anne Debant'
    ),
    (
        'TRIAP GAMMA', ' TRIO GEF2(cDNA)',
        'Anticancer', '16', 'VLLREGRGFKVRLVVV',
        'inhibit oncogenic rhogef', 'aptamer-Derived Peptide Inhibitors of Rho Guanine Nucleotide Exchange Factors Susanne Schmidt1 , Anne Debant'
    ),
    (
        'Pep8', 'Cdk2', 'Anticancer', '20',
        'YSFVHHGFFNFRVSWREMLA', 'inhibits H1 phosphorylation but not of pRB',
        'COLAS P, COHEN B, JESSEN T et al.: Genetic selection of peptide aptamers that recognize and inhibit cyclin-dependent kinase 2. Nature (1996) 380(6574):548-550'
    ),
    (
        'Pep104', 'Ras-V12', 'Anticancer',
        '20', 'FWQATLRLVSDKLILLYPDP', 'Prevents activation of Raf1',
        'XU CW, MENDELSOHN AR, BRENT R: Cells that register logical relationships among proteins. Proc. Natl. Acad. Sci. USA (1997) 94(23):12473-12478.'
    ),
    (
        'Pep8', 'Cdk2', 'Anticancer', '20',
        'YSFVHHGFFNFRVSWREMLA', 'Inhibits G1 to S phase transition',
        'COHEN BA, COLAS P, BRENT R: An artificial cell-cycle inhibitor isolated from a combinatorial library. Proc. Natl. Acad. Sci. USA (1998) 95(24):14272-14277'
    ),
    (
        'Pep8', 'DmCdk1', 'Anticancer', '20',
        'YSFVHHGFFNFRVSWREMLA', 'Eye defects during organogenesis',
        '(2004) 279(28):29589-29597. 89. KOLONIN MG, FINLEY RL JR: Targeting cyclin-dependent kinases in Drosophila with peptide aptamers. Proc. Natl. Acad. Sci. USA (1998) 95(24):14266-14271.'
    ),
    (
        'DK2pep2', 'DmCdk2', 'Anticancer',
        'NA', 'NA', 'Eye defects during organogenesis',
        '(2004) 279(28):29589-29597. 89. KOLONIN MG, FINLEY RL JR: Targeting cyclin-dependent kinases in Drosophila with peptide aptamers. Proc. Natl. Acad. Sci. USA (1998) 95(24):14266-14271.'
    ),
    (
        'Pep2(I)', 'Ste5', 'Anticancer', '13',
        'NA', '?-factor pheromone resistance',
        'CAPONIGRO G, ABEDI MR, HURLBURT AP et al.: Transdominant genetic analysis of a growth control pathway. Proc. Natl. Acad. Sci. USA (1998) 95(13):7508-7513.'
    ),
    (
        'AptC6', 'CBK1', 'Anticancer', 'NA',
        'NA', '?-factor pheromone resistance',
        'GEYER CR, COLMAN-LERNER A, BRENT R: ‘Mutagenesis’ by peptide aptamers identifies genetic network members and pathway connections. Proc. Natl. Acad. Sci. USA (1999) 96(15):8567-8572'
    ),
    (
        'P-7', 'Ste50', 'Anticancer', 'NA',
        'NA', '?-factor pheromone resistance',
        'NORMAN TC, SMITH DL, SORGER PK et al.: Genetic selection of peptide inhibitors of biological pathways. Science (1999) 285(5427):591-595'
    ),
    (
        'Apt5', 'E2F1', 'Anticancer', '19',
        'RCVRCRFVVWIGLRVRCLV', 'Inhibits entry into S-phase',
        'FABBRIZIO E, LE CAM L, POLANOWSKA J et al.: Inhibition of mammalian cell proliferation by genetically selected peptide aptamers that functionally antagonize E2F activity. Oncogene (1999) 18(30):4357-4363.'
    ),
    (
        'E61-1', 'HPV16-E6', 'Anticancer',
        '20', 'GALVHKLFSQTSGSCLVCIS', 'Blocks growth of HPV16+ cells, induction of apoptosis',
        'Induction of apoptosis in human papillomaviruspositive cancer cells by peptide aptamers targeting the viral E6 oncoprotein'
    ),
    (
        'TRXA101Q', 'ThyA', 'Anticancer',
        'NA', 'NA', 'Blocks thymidilate synthase causing thymine auxotrophy',
        'BLUM JH, DOVE SL, HOCHSCHILD A, MEKALANOS JJ: Isolation of peptide aptamers that inhibit intracellular processes. Proc. Natl. Acad. Sci. USA (2000) 97(5):2241-2246.'
    ),
    (
        'cRGDfV', 'Integrin', 'Anticancer',
        'NA', 'NA', '84% killing of LNCaP cells',
        'CHATTERJEE S, BRITE KH, MATSUMURA A: Induction of apoptosis of integrin-expressing human prostate cancer cells by cyclic Arg-Gly-Asp peptides. Clin. Cancer Res. (2001) 7(10):3006-3011'
    ),
    (
        'Pep-apt55', 'HPV16-E7', 'Anticancer',
        '20', 'LNFIFDERSDIYVLWLILEG', 'Induction of apoptosis in E7+ cells',
        'NAUENBURG S, ZWERSCHKE W, JANSEN-DURR P: Induction of apoptosis in cervical carcinoma cells by peptide aptamers that bind to the HPV-16 E7 oncoprotein. FASEB J. (2001) 15(3):592-594'
    ),
    (
        'C1-1', 'HBV core', 'Anticancer',
        '20', 'SFYSVLFLWGTCGGFSHSWY', 'Inhibition of viral capsid formation',
        'Peptide aptamers targeting the hepatitis B virus core protein: a new class of molecules with antiviral activity'
    ),
    (
        'HAGBP', 'Grb2', 'Anticancer', '11',
        'PPPPLPPRRRR', 'Inhibits cell proliferation',
        'KARDINAL C, KONKOL B, LIN H et al.: Chronic myelogenous leukemia blast cell proliferation is inhibited by peptides that disrupt Grb2-SoS complexes. Blood (2001) 98(6):1773-1781'
    ),
    (
        'PNC-21', 'p53', 'Anticancer', '9',
        'PPLSQETFS', 'Caused extensive tumour cell death',
        'KANOVSKY M, RAFFO A, DREW L et al.: Peptides from the amino terminal mdm-2-binding domain of p53, designed from conformational analysis, are selectively cytotoxic to transformed cells. Proc. Natl. Acad. Sci. USA (2001) 98(22):12438-12443.'
    ),
    (
        'TRIAP?', 'TRIO', 'Anticancer', '42',
        'AREGADGAICGYNLATLVMLGPSERVFCPLCEPCSSDIYELM',
        'Extension of neuritis in 45% of cells',
        'SCHMIDT S, DIRIONG S, MERY J, FABBRIZIO E, DEBANT A: Identification of the first Rho-GEF inhibitor, TRIPalpha, which targets the RhoA-specific GEF domain of Trio. FEBS Lett. (2002) 523(1-3):35-42'
    ),
    (
        'Pep141', 'Ras-V12', 'Anticancer',
        '39', 'WWGYLGKLFQTETDGALGPRPMCFILVEAVFASAACDSC',
        'Prevents activation of Raf1',
        'XU CW, LUO Z: Inactivation of Ras function by allele-specific peptide aptamers. Oncogene (2002) 21(37):5753-5757'
    ),
    (
        'RIP2', 'Ras-Val19', 'Anticancer',
        '6', 'LGQGSW', 'Induced growth and suppressed temperature sensitivity',
        'KURTZ SE, ESPOSITO K, TANG W, MENZEL R: Inhibition of an activated Ras protein with genetically selected peptide aptamers. Biotechnol. Bioeng. (2003) 82(1):38-46'
    ),
    (
        'KDI-1', 'EGFR', 'Anticancer', '20',
        'VFGVSWVVGFWCQMHRRLVC', 'Strongly reduced proliferation',
        'BUERGER C, NAGEL-WOLFRUM K, KUNZ C et al.: Sequence-specific peptide aptamers, interacting with the intracellular domain of the epidermal growth factor receptor, interfere with Stat3 activation and inhibit the growth of tumor cells. J. Biol. Chem. (2003) 278(39):37610-37621'
    ),
    (
        'PFKQ', 'XIAP', 'Anticancer', '6',
        'CPFKQC', '50% reduced cell viability, blocks caspase binding site',
        'TAMM I, TREPEL M, CARDO-VILA M et al.: Peptides targeting caspase inhibitors. J. Biol. Chem. (2003) 278(16):14401-14405.'
    ),
    (
        'DBD1', 'Stat3', 'Anticancer', 'NA',
        'NA', 'Reduced proliferation, induction of apoptosis',
        'NAGEL-WOLFRUM K, BUERGER C, WITTIG I et al.: The interaction of specific peptide aptamers with the DNA binding domain and the dimerization domain of transcription factor Stat3 inhibits transactivation and induces apoptosis in tumor cells. Mol. Cancer Res. (2004) 2(3):170-182.'
    ),
    (
        'PPC', 'Pro-MMP9', 'Anticancer', '18',
        'ADGACGYGRFSPPCGAAG', 'Inhibits cell invasion and migration up to 50%',
        'BJORKLUND M, HEIKKILA P, KOIVUNEN E: Peptide inhibition of catalytic and noncatalytic activities of matrix metalloproteinase-9 blocks tumor cell migration and invasion. J. Biol. Chem. (2004) 279(28):29589-29597'
    ),
    (
        'SHPR190', 'HIV-rev', 'Anticancer',
        'NA', 'NA', 'Inhibits HIV replication',
        'ROISIN A, ROBIN JP, DEREUDDRE-BOSQUET N et al.: Inhibition of HIV-1 replication by cell-penetrating peptides binding Rev. J. Biol. Chem. (2004) 279(10):9208-9214'
    ),
    (
        'Pentratin', 'NA', 'Anticancer', '16',
        'RQIKIWFQNRRMKWKK', 'penetrates into cell membrane',
        'DEROSSI D, JOLIOT AH, CHASSAING G, PROCHIANTZ A: The third helix of the Antennapedia homeodomain translocates through biological membranes. J. Biol. Chem. (1994) 269(14):10444-10450'
    ),
    (
        'Tat', '', 'Anticancer', '11', 'YGRKKRRQRRR',
        'promote the intracellular delivery of covalently bound proteins such as ?-galactosidase, RNase A, or horseradish peroxidase in several cell lines and tissues',
        'VIVES E, BRODIN P, LEBLEU B: A truncated HIV-1 Tat protein basic domain rapidly translocates through the plasma membrane and accumulates in the cell nucleus. J. Biol. Chem. (1997) 272(25):16010-16017'
    ),
    (
        'VP22', '', 'Anticancer', '34', 'DAATATRGRSAASRPTERPRAPARSASR PRRPVE',
        'intercellular transport', 'ELLIOTT G, O’HARE P: Intercellular trafficking and protein delivery by a herpesvirus structural protein. Cell (1997) 88(2):223-233'
    ),
    (
        'Transportan', '', 'Anticancer', '27',
        'GWTLNSAGYLLGKINLKALAALAKKIL',
        'In Bowes melanoma cells, transportan first localizes in the outer membrane and cytoplasmatic membrane structures. This is followed by a redistribution into the nuclear membrane and uptake into the nuclei where transportan concentrates in distinct substructures, probably the nucleoli.',
        'POOGA M, HALLBRINK M, ZORKO M, LANGEL U: Cell penetration by transportan. FASEB. J. (1998) 12(1):67-77'
    ),
    (
        'MAP', '', 'Anticancer', '18', 'KLALKLALKALKAALKLA',
        'alpha-helical amphipathic model peptide involved in the uptake into mammalian cells ',
        'OEHLKE J, SCHELLER A, WIESNER B et al.: Cellular uptake of an alpha-helical amphipathic model peptide with the potential to deliver polar compounds into the cell interior non-endocytically. Biochim. Biophys. Acta (1998) 1414(1-2):127-139.'
    ),
    (
        'TP10', '', 'Anticancer', '21', 'AGYLLGKINLKALAALAKKIL',
        'penetrates into cell membrane, no effect on GTPase activity in Rin m5F cell membranes.',
        'SOOMETS U, LINDGREN M, GALLET X et al.: Deletion analogues of transportan. Biochim. Biophys. Acta (2000) 1467(1):165-176.'
    ),
    (
        '9R', '', 'Anticancer', '9', 'RRRRRRRRR',
        'enhance cellular uptake', 'WENDER PA, MITCHELL DJ, PATTABIRAMAN K et al.: The design, synthesis, and evaluation of molecules that enable or enhance cellular uptake: peptoid molecular transporters. Proc. Natl. Acad. Sci. USA (2000) 97(24):13003-13008'
    ),
    (
        'PTD-5', '', 'Anticancer', '12', 'RRQRRTSKLMKR',
        'helps in protein transduction',
        'MI Z, MAI J, LU X, ROBBINS PD: Characterization of a class of cationic peptides able to facilitate efficient protein transduction in vitro and in vivo. Mol. Ther.(2000) 2(4):339-347'
    ),
    (
        'pVEC', '', 'Anticancer', '18', 'LLIILRRRIRKQAHAHSK',
        'helps in internalization of pVEC',
        'ELMQUIST A, LINDGREN M, BARTFAI T, LANGEL U: VE-cadherinderived cell-penetrating peptide, pVEC, with carrier functions. Exp. Cell Res. (2001) 269(2):237-244'
    ),
    (
        'C6-14', 'bind to Nr-13 protein ',
        'Anticancer', '20', 'RRGSPVLEASKAVDVLFLVL',
        'inhibit Nr-13 protein which is apoptosis regulator',
        'Modulation of Nr-13 antideath activity by peptide aptamers A-L Nouvion1,3, J Thibaut1,4, OD Lohez1,4,5, S Venet1 , P Colas2 , GGillet1 and P Lalle1'
    ),
    (
        'M10b1', 'bind to Nr-13 protein  ',
        'Anticancer', '20', 'RRGSPVMEASKAVDVLFLAL',
        'inhibit Nr-13 protein which is apoptosis regulator',
        'Modulation of Nr-13 antideath activity by peptide aptamers A-L Nouvion1,3, J Thibaut1,4, OD Lohez1,4,5, S Venet1 , P Colas2 , GGillet1 and P Lalle1'
    ),
    (
        'B16', 'bind to Nr-13 and Bax', 'Anticancer',
        '20', 'PRGAPMWMRWVCQMLETMFL', 'inhinbit Nr-13 and Bax',
        'Modulation of Nr-13 antideath activity by peptide aptamers A-L Nouvion1,3, J Thibaut1,4, OD Lohez1,4,5, S Venet1 , P Colas2 , GGillet1 and P Lalle1'
    ),
    (
        'B16m3', 'bind to Nr-13 protein  ',
        'Anticancer', '20', 'PRGAPLWLCWVWQMLETMLP ',
        'inhibit Nr-13 protein which is apoptosis regulator',
        'Modulation of Nr-13 antideath activity by peptide aptamers A-L Nouvion1,3, J Thibaut1,4, OD Lohez1,4,5, S Venet1 , P Colas2 , GGillet1 and P Lalle1'
    ),
    (
        'AS1411', 'bind to nucleolin, ',
        'Anticancer', '26', 'GGTGGTGGTGGTTGTGGTGGTGGTGG',
        'macropinocytosis in cancer cells ',
        'Peptide-aptamer Co-Assembly Nanocarrier for Cancer Therapy Yue Ma, ‡ a Wenjun Li, ‡ a Ziyuan Zhou, ab Xuan Qin, a Dongyuan Wang, a Yubo Gao, c Zhiqiang Yu, d Feng Yin *a and Zigang Li *a'
    ),
    (
        'AS1411', 'in 2nd phase trail', 'Anticancer',
        'NA', 'NA', '', 'A new paradigm for aptamer therapeutic AS1411 action: Uptake by macropinocytosis and its stimulation by a nucleolin-dependent mechanism.E. Merit Reyes-Reyes,1 Yun Teng,1 and Paula J. Bates1,2'
    ),
    (
        'TRIPalpha', 'Rho-GEF inhibitor',
        'Anticancer', '42', 'AREGADGAICGYNLATLVMLGPSERVFCPLCEPCSSDIYELM',
        'inhibit Rho-GEF', 'Identi¢cation of the first Rho^GEF inhibitor, TRIPK, which targets the RhoA-specific GEF domain of Trio, Susanne Schmidta, Sylvie Dirionga, Jean Me¤rya, Eric Fabbriziob, Anne Debanta'
    ),
    (
        'A17', 'inhibit hsp70', 'Anticancer',
        '13', 'YCAYYSPRHKTTF ', 'increase sensitivity of tumor cells against anti cancer drugs',
        'The Hsp70 inhibiting peptide aptamer A17 potentiates Q3 radiosensitization of tumor cells by Hsp90 inhibition Q2 Daniela Schilling a, b, * , Carmen Garrido c, d , Stephanie Combs a, b , Gabriele Multhof'
    ),
    (
        'A8', 'inhibit hsp70', 'Anticancer',
        '8', 'SPWPRPTY ', 'increase sensitivity of tumor cells against anti cancer drugs, A8 and hsp70 association blocks the ability of exosomes to activate MDSCs',
        'The Hsp70 inhibiting peptide aptamer A17 potentiates Q3 radiosensitization of tumor cells by Hsp90 inhibition '
    ),
    (
        'A11', 'inhibit hsp70', 'Anticancer',
        '13', 'CIWVSDGKKLWRH ', 'increase sensitivity of tumor cells against anti cancer drugs',
        ''
    ),
    (
        'A12', 'inhibit hsp70', 'Anticancer',
        '13', 'CYTQYRKCQELTA ', 'increase sensitivity of tumor cells against anti cancer drugs',
        ''
    ),
    (
        'rs3-PA', 'interacts with the dimerization domain of STAT3',
        'Anticancer', '20', 'VRH SAL HMA VGP LSW PAR VS',
        'inhibition of stat3 , inhibits the migration and the angiogenic induction potential of cultured tumor cells, useful in skin cancer',
        'Inhibition of Stat3 by peptide aptamer rS3-PA enhances growth suppressive effects of irinotecan on colorectal cancer cells,A membrane penetrating peptide aptamer inhibits STAT3 function and suppresses the growth of STAT3 addicted tumor cells'
    ),
    (
        ' Id1/3-PA7 ', 'ovarian cancer cells ES-2 and PA-1',
        'Anticancer', 'NA', 'NA', 'cell-cycle arrest and apoptosis in ovarian cancer cells ES-2 and PA-1',
        'Inhibition of Id proteins by a peptide aptamer induces cell-cycle arrest and apoptosis in ovarian cancer cells'
    ),
    (
        'EE-02', 'cap-binding site of eIF4E',
        'Anticancer', '11', 'AC?EMGFFQDC?G',
        'eIF4F complex disruption', 'Development of a novel peptide aptamer that interacts with the eIF4E capped-mRNA binding site using peptide epitope linker evolution (PELE)'
    ),
    (
        'VH-DiFCAP-01', 'modulate known biological activities of eIF4E',
        'Anticancer', '15', 'PLPE?M?G?F?F?TNIPAMV',
        'recapitulates the interactions of EE-02 with the cap-binding site and forms additional interactions',
        'Development of a novel peptide aptamer that interacts with the eIF4E capped-mRNA binding site using peptide epitope linker evolution (PELE)'
    ),
    (
        'Apt48', 'BCL-6', 'Anticancer', '10',
        'HGPRDWCLFG', 'inhibit Bcl-6 ( bcl-6 allows cell proliferation and its constitutive expression promotes growth of lymphomas)',
        'A peptide aptamer to antagonize BCL-6 function'
    ),
    (
        'DUP-1', 'prostate-specific membrane antigen (PSMA)-negative cells',
        'Anticancer', '12', 'FRPNRAQDYNTN',
        'produce the dual-aptamer with RNA aptamer A10-3.2 modified tumor targeting gene and DOX delivery system',
        'Enhanced growth inhibition of prostate cancer in vitro and in vivo by a recombinant adenovirus-mediated dual-aptamer modified drug delivery system'
    ),
    (
        'p42', 'bind SoX2', 'Anticancer',
        '11', 'FSTLFFPLFFL', 'inhibit SoX2 which is found in high level in Esophageal squamous cell carcinoma (ESCC) , reduce priliferation and migration of ESCC  cells',
        'Targeting SOX2 Protein with Peptide Aptamers for Therapeutic Gains against Esophageal Squamous Cell Carcinoma'
    ),
    (
        'AII-7', 'ErBb2 receptor', 'Anticancer',
        '41', 'LNFYRHGFLPNAVMASMLEVGPWFELLGLCGLAGHPLSSLR',
        'inhibited the induction of AKT kinase in MCF7 breast cancer cells',
        'Peptide Aptamers with Binding Specificity for the Intracellular Domain of the ErbB2 Receptor Interfere with AKT Signaling and Sensitize Breast Cancer Cells to Taxol'
    ),
    (
        'Ag-11', 'ErBb2 receptor', 'Anticancer',
        '21', 'GLAHGVAIYTELPLTRMARGG', 'inhibit EGFR signqlling',
        'Peptide Aptamers with Binding Specificity for the Intracellular Domain of the ErbB2 Receptor Interfere with AKT Signaling and Sensitize Breast Cancer Cells to Taxol'
    ),
    (
        'S5-DBD-PA', 'DNA-Binding domain of Stat-5',
        'Anticancer', '12', 'QGSWTINISKRL',
        'stat5 degradation which is expressed in carcinomas, breast cancer and prostate cancer',
        'The Inhibition of Stat5 by a Peptide aptamer Ligand Specific for the DNA Binding Domain Prevents Target Gene Transactivation and the Growth of Breast and Prostate Tumor Cells'
    ),
    (
        'TRIPE32G', 'inhibits Tgat GEF',
        'Anticancer', '42', 'AREGADGAICGYNLATLVMLGPSERVFCPLCGPCSSDIYELM',
        'reduces Tgat induced RhoA activation and foci formation',
        'aptamer-Derived Peptides as Potent Inhibitors of the Oncogenic RhoGEF Tgat'
    ),
    (
        'pep6', 'Cdk2', 'Anticancer', '20',
        'RVKLGYSFWAQSLLRCISVG', 'Detect cdks whose activity is important in proliferating and cancerous cells',
        'Peptide Aptamers in Label-Free Protein Detection: 2. Chemical Optimization and Detection of Distinct Protein Isoforms'
    ),
    (
        'pep9', 'CDK2', 'Anticancer', '30',
        'QQRFVFSPSWFTCAGTSDFWGPEPLFDWTR',
        'detect cdks whose activity is important in proliferating and cancerous cells',
        'Peptide Aptamers in Label-Free Protein Detection: 2. Chemical Optimization and Detection of Distinct Protein Isoforms'
    ),
    (
        'Trx-SARA', 'Smad2 and Smad3', 'Anticancer',
        '14', 'TIPPLQQAQASGAL', 'reduce the level of Smad2 and Smad3 in complex with Smad4 after TGF-?1 stimulation',
        'Inhibition of Transforming Growth Factor-?1–induced Signaling and Epithelial-to-Mesenchymal Transition by the Smad-binding Peptide aptamer Trx-SARA'
    ),
    (
        'NHBP-1', ': Hydrophobic single-wall carbon nanohorns (SWNHs)',
        'Anticancer', '12', 'DYFSSPYYEQLF',
        'Dispersion of Cisplatin-Loaded Carbon Nanohorns',
        'Dispersion of Cisplatin-Loaded Carbon Nanohorns with a Conjugate Comprised of an Artificial Peptide aptamer and Polyethylene Glycol'
    ),
    (
        'Swiggle', 'intracellular domain of membrane type 1-metalloproteinase (MT1-MMP)',
        'Anticancer', '10', 'GGLIPCYFMH',
        'inhibits MT1-MMP internalization',
        'Peptide aptamers as new tools to modulate clathrin-mediated internalisation — inhibition of MT1-MMP internalisation'
    ),
    (
        'A13', 'HSP70', 'Anticancer', 'NA',
        'NA', 'enhance the accumulation efficiency of HBP-based nanomedicines in vivo',
        ''
    ),
    (
        'FITC', 'bind to hcg', 'Anticancer',
        '12', 'PPLRINRHILTR', 'bind to hcg which is found in high level in some cancers like prostate cancer, testicular cancer',
        'A Graphene Oxide-Based Fluorescent Method for the Detection of Human Chorionic Gonadotropin'
    ),
    (
        'p1', 'CK2b-interacting peptide',
        'Anticancer', '19', 'GKMNGVLPLAW-PSLYLRLP',
        'target CK2b can induce apoptosis in mammalian cells through the recruitment of a p53-dependent apoptosis pathway',
        'p53-dependent inhibition of mammalian cell survival bya genetically selected peptide aptamer that targets the regulatorysubunit of protein kinase CK2'
    ),
    (
        'HPV-16-L1', 'human papilloma virus(HPV)',
        'Anticancer', '15', 'NLASSNYFPTPSGSM',
        'actb as affinity capture reagent for detection of HPV',
        'Development of interdigitated arrays coated with functional polyaniline/MWCNT for electrochemical biodetection: Application for human papilloma virus'
    ),
    (
        'PA11', 'HSP70', 'Anticancer', '13',
        'QLSGWVGRCLNIN', 'INHIBIT HSP27 WHICH IS ANTI APOPTOTIC',
        'Inhibition of heat shock protein 27 (HspB1) tumorigenic functions by peptide aptamers'
    ),
    (
        'PA50', 'HSP70', 'Anticancer', '8',
        'Y L L R R L C C', 'INHIBIT HSP27 WHICH IS ANTI APOPTOTIC',
        'Inhibition of heat shock protein 27 (HspB1) tumorigenic functions by peptide aptamers'
    ),
    (
        'F79', 'RAD52', 'Anticancer', '13',
        'VINLANEMFGYNG', 'synthetic lethality in breast, pancreatic, and ovarian carcinoma cells harboring BRCA1- or BRCA2-inactivating mutations',
        'Personalized synthetic lethality induced by targeting RAD52 in leukemias identified by gene mutation and expression profile'
    ),
    (
        'Y315', 'RAD51', 'Anticancer', '16',
        ' ETRICKIpYDSPCLLEA', 'target Rad51 which is directly phosphorylated by BCR-ABL1 oncogenic kinase, preferentially detected in the nuclei of leukemia',
        'Targeting RAD51 phosphotyrosine-315 to prevent unfaithful recombination repair in BCR-ABL1 leukemia'
    ),
    (
        'CRV', 'Pro-MMP9', 'Anticancer', '18',
        'ADGACRVYGPYLLCGAAG', 'Inhibits cell invasion and migration up to 50%',
        'BJORKLUND M, HEIKKILA P, KOIVUNEN E: Peptide inhibition of catalytic and noncatalytic activities of matrix metalloproteinase-9 blocks tumor cell migration and invasion. J. Biol. Chem. (2004) 279(28):29589-29597'
    ),
    (
        'Pept+A2:G11ide aptamer-72', 'Central domain of A20 protein ',
        'Antiviral', '8', 'RQLVLRTR', 'Binds with DNA Polymerase E9 and stops the viral DNA Polymerase activity.',
        'Saccucci, L., Crance, J.-M., Colas, P., Bickle, M., Garin, D., & Iseni, F. (2009). Inhibition of vaccinia virus replication by peptide aptamers. Antiviral Research, 82(3), 134–140'
    ),
    (
        'No name', 'Receptor binding domain (RBD) of S-protein of CoV-2',
        'Antiviral', '18', 'AKTFLDKFNHEADLFYQ',
        ' It mimics the ACE2 receptor due to similar binding sequences.',
        'Devi, A., & Chaitanya, N. S. N. (2022). Designing of peptide aptamer targeting the receptor-binding domain of spike protein of SARS-CoV-2: An in silico study. Molecular Diversity, 26(1), 157–169.'
    ),
    (
        'PA34', ' DHBV core protein', 'Antiviral',
        '20', 'ASLGLRPMGWRRSMCMAMLR', 'forms coaggregates and transported to aggresomes.',
        'Tomai, E., Butz, K., Lohrey, C., Weizsäcker, F. von, Zentgraf, H., & Hoppe-Seyler, F. (2006). Peptide aptamer-mediated Inhibition of Target Proteins by Sequestration into Aggresomes *. Journal of Biological Chemistry, 281(30), 21345–21352'
    ),
    (
        'LEDGF/p75', 'HIV-1 Integrase', 'Antiviral',
        '19', 'WQCLTLTHRGFVLLTITVL', ' Binds to HIV-1 IN enzyme and strongly inhibits the dimerization of IN protein.',
        'Armon-Omer, A., Levin, A., Hayouka, Z., Butz, K., Hoppe-Seyler, F., Loya, S., Hizi, A., Friedler, A., & Loyter, A. (2008). Correlation Between Shiftide Activity and HIV-1 Integrase Inhibition by a Peptide Selected from a Combinatorial Library. Journal of Molecular Biology, 376(4), 971–982.'
    ),
    (
        'A4', 'puL84 NLS', 'Antiviral', '20',
        'SSCNMGWDTPACCVWFPYWV', 'Interacts with the open reading frame of uL84 causing interference in protein interactions. ',
        'Inhibition of Human Cytomegalovirus Replication via Peptide Aptamers Directed against the Nonconventional Nuclear Localization Signal of the Essential Viral Replication Factor pUL84 | Journal of Virology. (n.d.). Retrieved June 19, 2023'
    ),
    (
        'A8', 'puL84 NLS', 'Antiviral', '20',
        'MAVGLVLCDWWLGEYLLELA', 'Interacts with the open reading frame of uL84 causing interference in protein interactions. ',
        'Inhibition of Human Cytomegalovirus Replication via Peptide Aptamers Directed against the Nonconventional Nuclear Localization Signal of the Essential Viral Replication Factor pUL84 | Journal of Virology. (n.d.). Retrieved June 19, 2023'
    ),
    (
        'A56', 'puL84 NLS', 'Antiviral', '20',
        'PVLQPALSLSCGPEPLLLSC', 'Interacts with the open reading frame of uL84 causing interference in protein interactions. ',
        'Inhibition of Human Cytomegalovirus Replication via Peptide Aptamers Directed against the Nonconventional Nuclear Localization Signal of the Essential Viral Replication Factor pUL84 | Journal of Virology. (n.d.). Retrieved June 19, 2023'
    ),
    (
        'A110', 'puL84 NLS', 'Antiviral',
        '20', 'IEVTFVNRRGDGAELWYLSA', 'Interacts with the open reading frame of uL84 causing interference in protein interactions. ',
        'Inhibition of Human Cytomegalovirus Replication via Peptide Aptamers Directed against the Nonconventional Nuclear Localization Signal of the Essential Viral Replication Factor pUL84 | Journal of Virology. (n.d.). Retrieved June 19, 2023'
    ),
    (
        'C1-1', 'HBV core protein', 'Antiviral',
        '20', 'SFYSVLFLWGTCGGFSHSWY', 'C1-1 binds to the HBV core protein and the HBV long-surface antigen L-HbsAg during envelopement of capsid.',
        'Butz, K., Denk, C., Fitscher, B., Crnkovic-Mertens, I., Ullmann, A., Schröder, C. H., & Hoppe-Seyler, F. (2001). Peptide aptamers targeting the hepatitis B virus core protein: A new class of molecules with antiviral activity. Oncogene, 20(45), Article 45'
    ),
    (
        'AmPep1', 'Replication origin loop structure of TYLCV',
        'Antiviral', '16', 'SVGRKWRMKWAQMRQQ',
        'Interacts with the replication origin loop structure of the TYLCV',
        'Mendoza-Figueroa, J. S., Kvarnheden, A., Méndez-Lozano, J., Rodríguez-Negrete, E.-A., Arreguín-Espinosa de los Monteros, R., & Soriano-García, M. (2018). A peptide derived from enzymatic digestion of globulins from amaranth shows strong affinity binding to the replication origin of Tomato yellow leaf curl virus reducing viral replication in Nicotiana benthamiana. Pesticide Biochemistry and Physiology, 145, 56–65'
    ),
    (
        'P6.1 (Detector peptide)', 'ZIKV envelope protein',
        'Diagnostic', '12', 'KQERNNWPLTWT',
        'These aptamers bind to the ZIKV envelope protein with strong binding affinity.',
        'Nguyen, A. T. V., Duong, B. T., Park, H., & Yeo, S.-J. (2022). Development of a peptide aptamer pair-linked rapid fluorescent diagnostic system for Zika virus detection. Biosensors and Bioelectronics, 197, 113768'
    ),
    (
        'P29.1', 'ZIKV envelope protein',
        'Diagnostic', '11', 'KYTTSTLKSGV',
        'These aptamers bind to the ZIKV envelope protein with strong binding affinity.',
        'Nguyen, A. T. V., Duong, B. T., Park, H., & Yeo, S.-J. (2022). Development of a peptide aptamer pair-linked rapid fluorescent diagnostic system for Zika virus detection. Biosensors and Bioelectronics, 197, 113768'
    ),
    (
        'B2.33', 'ZIKV envelope protein',
        'Diagnostic', '15', 'KRHVWVSLSYSCAEA',
        'These aptamers bind to the ZIKV envelope protein with strong binding affinity.',
        'Nguyen, A. T. V., Duong, B. T., Park, H., & Yeo, S.-J. (2022). Development of a peptide aptamer pair-linked rapid fluorescent diagnostic system for Zika virus detection. Biosensors and Bioelectronics, 197, 113768'
    ),
    (
        'P1', 'Complementary determining Regions(CDRs) of H5 viral subtype',
        'Diagnostic', '10', 'KASGYTFTSF ',
        'Docking of P1 on H5-CDR showed effective binding (Highest binding energy).',
        'Nguyen, A. T. V., Trinh, T. T. T., Hoang, V. T., Dao, T. D., Tuong, H. T., Kim, H. S., Park, H., & Yeo, S.-J. (2019). Peptide aptamer of Complementarity-determining Region to Detect Avian Influenza Virus. Journal of Biomedical Nanotechnology, 15(6), 1185–1200'
    ),
    (
        'HPV-16-L1', 'Anti HPV-16 antibody',
        'Diagnostic', '15', 'NLASSNYFPTPSGSM',
        'Antigen-antibody interaction took place on electrode surface.',
        'Tran, L. D., Nguyen, D. T., Nguyen, B. H., Do, Q. P., & Le Nguyen, H. (2011). Development of interdigitated arrays coated with functional polyaniline/MWCNT for electrochemical biodetection: Application for human papilloma virus. Talanta, 85(3), 1560–1565'
    ),
    (
        ' Z_10.8', 'ZIKV envelope protein',
        'Diagnostic', '10', 'KRAVVSCAEA',
        'Z_10.8 peptide has a higher binding affinity than the other candidates tested.',
        'Kim, D. T. H., Bao, D. T., Park, H., Ngoc, N. M., & Yeo, S.-J. (2018). Development of a novel peptide aptamer-based immunoassay to detect Zika virus in serum and urine. Theranostics, 8(13), 3629–3642'
    ),
    (
        'Antibody derived Peptide aptamer (ADPA)',
        'Pro-inflammatory cytokine IL-6',
        'Diagnostic', '36', 'CNRAGWGMGDCGSGSGCGFTFSSYRCGPGSAGGGSC',
        'The three CDR loops of 61H7 heavy chain of ADPA are in contact with IL-6, contributing to the binding specificity and affinity. ',
        'He, J., Zhou, L., Huang, G., Shen, J., Chen, W., Wang, C., Kim, A., Zhang, Z., Cheng, W., Dai, S., Ding, F., & Chen, P. (2022). Enhanced Label-Free Nanoplasmonic Cytokine Detection in SARS-CoV-2 Induced Inflammation Using Rationally Designed Peptide aptamer. ACS Applied Materials & Interfaces, 14(43), 48464–48475'
    ),
    (
        'E61-1', 'E6 oncoprotein of HPV-16 infected cervical keratinocytes',
        'Diagnostic', '20', 'GALVHKLFSQTSGSCLVCIS',
        'Binds to the E6 oncoprotein of HPV-16 in microarray format.',
        'Development of peptide aptamer microarrays for detection of HPV16 oncoproteins in cell extracts—ScienceDirect. (n.d.). Retrieved June 19, 2023'
    ),
    (
        'E7-4', 'E7 oncoprotein of HPV-16 infected cervical keratinocytes',
        'Diagnostic', '20', 'YLRGSRLVCWGPASCPCFVY',
        'Binds to the E7 oncoprotein of HPV-16 in microarray format.',
        'Development of peptide aptamer microarrays for detection of HPV16 oncoproteins in cell extracts—ScienceDirect. (n.d.). Retrieved June 19, 2023'
    );