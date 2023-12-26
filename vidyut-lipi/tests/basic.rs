use vidyut_lipi::Scheme::*;
use vidyut_lipi::{transliterate, Scheme};

/// Transliterates all input strings against each other.
///
/// This function assums that all strings provided are lossless.
fn assert_exhaustive_pairwise(examples: &[(Scheme, &str)]) {
    for (from, input) in examples {
        for (to, expected) in examples {
            let actual = transliterate(input, *from, *to);
            assert_eq!(*expected, actual, "t(\"{input}\", {from:?}, {to:?})");
        }
    }
}

#[test]
fn vowels() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "a A i I u U R RR lR lRR e ai o au"),
        (Iast, "a ā i ī u ū ṛ ṝ ḷ ḹ e ai o au"),
        (Itrans, "a A i I u U RRi RRI LLi LLI e ai o au"),
        (Slp1, "a A i I u U f F x X e E o O"),
        (Velthuis, "a aa i ii u uu .r .R .l .L e ai o au"),
        // Indic
        (Bengali, "অ আ ই ঈ উ ঊ ঋ ৠ ঌ ৡ এ ঐ ও ঔ"),
        (Brahmi, "𑀅 𑀆 𑀇 𑀈 𑀉 𑀊 𑀋 𑀌 𑀍 𑀎 𑀏 𑀐 𑀑 𑀒"),
        (Devanagari, "अ आ इ ई उ ऊ ऋ ॠ ऌ ॡ ए ऐ ओ औ"),
        (Grantha, "𑌅 𑌆 𑌇 𑌈 𑌉 𑌊 𑌋 𑍠 𑌌 𑍡 𑌏 𑌐 𑌓 𑌔"),
        (Gujarati, "અ આ ઇ ઈ ઉ ઊ ઋ ૠ ઌ ૡ એ ઐ ઓ ઔ"),
        (Kannada, "ಅ ಆ ಇ ಈ ಉ ಊ ಋ ೠ ಌ ೡ ಏ ಐ ಓ ಔ"),
        (Malayalam, "അ ആ ഇ ഈ ഉ ഊ ഋ ൠ ഌ ൡ ഏ ഐ ഓ ഔ"),
        (Oriya, "ଅ ଆ ଇ ଈ ଉ ଊ ଋ ୠ ଌ ୡ ଏ ଐ ଓ ଔ"),
        (Sinhala, "අ ආ ඉ ඊ උ ඌ ඍ ඎ ඏ ඐ ඒ ඓ ඕ ඖ"),
        (Telugu, "అ ఆ ఇ ఈ ఉ ఊ ఋ ౠ ఌ ౡ ఏ ఐ ఓ ఔ"),
    ]);
}

/// Includes vowel marks and virama
#[test]
fn dependent_marks() {
    assert_exhaustive_pairwise(&[
        (
            HarvardKyoto,
            "ka kA ki kI ku kU kR kRR klR klRR ke kai ko kau k",
        ),
        (Iast, "ka kā ki kī ku kū kṛ kṝ kḷ kḹ ke kai ko kau k"),
        (
            Itrans,
            "ka kA ki kI ku kU kRRi kRRI kLLi kLLI ke kai ko kau k",
        ),
        (Slp1, "ka kA ki kI ku kU kf kF kx kX ke kE ko kO k"),
        // Indic
        (Bengali, "ক কা কি কী কু কূ কৃ কৄ কৢ কৣ কে কৈ কো কৌ ক্"),
        (Brahmi, "𑀓 𑀓𑀸 𑀓𑀺 𑀓𑀻 𑀓𑀼 𑀓𑀽 𑀓𑀾 𑀓𑀿 𑀓𑁀 𑀓𑁁 𑀓𑁂 𑀓𑁃 𑀓𑁄 𑀓𑁅 𑀓𑁆"),
        (Devanagari, "क का कि की कु कू कृ कॄ कॢ कॣ के कै को कौ क्"),
        (Grantha, "𑌕 𑌕𑌾 𑌕𑌿 𑌕𑍀 𑌕𑍁 𑌕𑍂 𑌕𑍃 𑌕𑍄 𑌕𑍢 𑌕𑍣 𑌕𑍇 𑌕𑍈 𑌕𑍋 𑌕𑍗 𑌕𑍍"),
        (Gujarati, "ક કા કિ કી કુ કૂ કૃ કૄ કૢ કૣ કે કૈ કો કૌ ક્"),
        (Kannada, "ಕ ಕಾ ಕಿ ಕೀ ಕು ಕೂ ಕೃ ಕೄ ಕೢ ಕೣ ಕೇ ಕೈ ಕೋ ಕೌ ಕ್"),
        (Malayalam, "ക കാ കി കീ കു കൂ കൃ കൄ കൢ കൣ കേ കൈ കോ കൌ ക്"),
        (Oriya, "କ କା କି କୀ କୁ କୂ କୃ କୄ କୢ କୣ କେ କୈ କୋ କୌ କ୍"),
        (Sinhala, "ක කා කි කී කු කූ කෘ කෲ කෟ කෳ කේ කෛ කෝ කෞ ක්"),
        (Telugu, "క కా కి కీ కు కూ కృ కౄ కౢ కౣ కే కై కో కౌ క్"),
    ]);
}

#[test]
fn ayogavahas() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "aM aH"),
        (Iast, "aṃ aḥ"),
        (Itrans, "aM aH"),
        (Slp1, "aM aH"),
        (Velthuis, "a.m a.h"),
        // Indic
        (Devanagari, "अं अः"),
        (Kannada, "ಅಂ ಅಃ"),
        (Malayalam, "അം അഃ"),
        (Telugu, "అం అః"),
    ]);
}

#[test]
fn consonants() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "ka kha ga gha Ga ca cha ja jha Ja Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va za Sa sa ha La",),
        (Iast, "ka kha ga gha ṅa ca cha ja jha ña ṭa ṭha ḍa ḍha ṇa ta tha da dha na pa pha ba bha ma ya ra la va śa ṣa sa ha ḻa"),
        (Itrans, "ka kha ga gha ~Na cha Cha ja jha ~na Ta Tha Da Dha Na ta tha da dha na pa pha ba bha ma ya ra la va sha Sha sa ha La"),
        (Slp1, "ka Ka ga Ga Na ca Ca ja Ja Ya wa Wa qa Qa Ra ta Ta da Da na pa Pa ba Ba ma ya ra la va Sa za sa ha La"),
        (Velthuis, "ka kha ga gha \"na ca cha ja jha ~na .ta .tha .da .dha .na ta tha da dha na pa pha ba bha ma ya ra la va \"sa .sa sa ha La"),
        // Indic
        (Brahmi, "𑀓 𑀔 𑀕 𑀖 𑀗 𑀘 𑀙 𑀚 𑀛 𑀜 𑀝 𑀞 𑀟 𑀠 𑀡 𑀢 𑀣 𑀤 𑀥 𑀦 𑀧 𑀨 𑀩 𑀪 𑀫 𑀬 𑀭 𑀮 𑀯 𑀰 𑀱 𑀲 𑀳 𑀴"),
        (Devanagari, "क ख ग घ ङ च छ ज झ ञ ट ठ ड ढ ण त थ द ध न प फ ब भ म य र ल व श ष स ह ळ"),
        (Grantha, "𑌕 𑌖 𑌗 𑌘 𑌙 𑌚 𑌛 𑌜 𑌝 𑌞 𑌟 𑌠 𑌡 𑌢 𑌣 𑌤 𑌥 𑌦 𑌧 𑌨 𑌪 𑌫 𑌬 𑌭 𑌮 𑌯 𑌰 𑌲 𑌵 𑌶 𑌷 𑌸 𑌹 𑌳"),
        (Gujarati, "ક ખ ગ ઘ ઙ ચ છ જ ઝ ઞ ટ ઠ ડ ઢ ણ ત થ દ ધ ન પ ફ બ ભ મ ય ર લ વ શ ષ સ હ ળ"),
        (Kannada, "ಕ ಖ ಗ ಘ ಙ ಚ ಛ ಜ ಝ ಞ ಟ ಠ ಡ ಢ ಣ ತ ಥ ದ ಧ ನ ಪ ಫ ಬ ಭ ಮ ಯ ರ ಲ ವ ಶ ಷ ಸ ಹ ಳ"),
        (Malayalam, "ക ഖ ഗ ഘ ങ ച ഛ ജ ഝ ഞ ട ഠ ഡ ഢ ണ ത ഥ ദ ധ ന പ ഫ ബ ഭ മ യ ര ല വ ശ ഷ സ ഹ ള"),
        (Oriya, "କ ଖ ଗ ଘ ଙ ଚ ଛ ଜ ଝ ଞ ଟ ଠ ଡ ଢ ଣ ତ ଥ ଦ ଧ ନ ପ ଫ ବ ଭ ମ ଯ ର ଲ ଵ ଶ ଷ ସ ହ ଳ"),
        (Sinhala, "ක ඛ ග ඝ ඞ ච ඡ ජ ඣ ඤ ට ඨ ඩ ඪ ණ ත ථ ද ධ න ප ඵ බ භ ම ය ර ල ව ශ ෂ ස හ ළ"),
        (Telugu, "క ఖ గ ఘ ఙ చ ఛ జ ఝ ఞ ట ఠ డ ఢ ణ త థ ద ధ న ప ఫ బ భ మ య ర ల వ శ ష స హ ళ"),
    ]);
}

#[test]
fn symbols() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Iast, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Itrans, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        (Slp1, "0 1 2 3 4 5 6 7 8 9 . .. '"),
        (Velthuis, "0 1 2 3 4 5 6 7 8 9 | || .a"),
        // Indic
        (Bengali, "০ ১ ২ ৩ ৪ ৫ ৬ ৭ ৮ ৯ । ॥ ঽ"),
        (Devanagari, "० १ २ ३ ४ ५ ६ ७ ८ ९ । ॥ ऽ"),
        (Grantha, "௦ ௧ ௨ ௩ ௪ ௫ ௬ ௭ ௮ ௯ । ॥ 𑌽"),
        (Gujarati, "૦ ૧ ૨ ૩ ૪ ૫ ૬ ૭ ૮ ૯ । ॥ ઽ"),
        (Gurmukhi, "੦ ੧ ੨ ੩ ੪ ੫ ੬ ੭ ੮ ੯ । ॥ ऽ"),
        (Kannada, "೦ ೧ ೨ ೩ ೪ ೫ ೬ ೭ ೮ ೯ । ॥ ಽ"),
        (Malayalam, "൦ ൧ ൨ ൩ ൪ ൫ ൬ ൭ ൮ ൯ । ॥ ഽ"),
        (Oriya, "୦ ୧ ୨ ୩ ୪ ୫ ୬ ୭ ୮ ୯ । ॥ ଽ"),
        (Telugu, "౦ ౧ ౨ ౩ ౪ ౫ ౬ ౭ ౮ ౯ । ॥ ఽ"),
    ]);
}

#[test]
fn basic_sentences() {
    assert_exhaustive_pairwise(&[
        (HarvardKyoto, "nArAyaNaM namaskRtya naraM caiva narottamam . devIM sarasvatIM caiva tato jayamudIyaret .. 1 ..",),
        (Iast, "nārāyaṇaṃ namaskṛtya naraṃ caiva narottamam . devīṃ sarasvatīṃ caiva tato jayamudīyaret .. 1 .."),
        (Itrans, "nArAyaNaM namaskRRitya naraM chaiva narottamam | devIM sarasvatIM chaiva tato jayamudIyaret || 1 ||"),
        (Slp1, "nArAyaRaM namaskftya naraM cEva narottamam . devIM sarasvatIM cEva tato jayamudIyaret .. 1 .."),
        (Velthuis, "naaraaya.na.m namask.rtya nara.m caiva narottamam | devii.m sarasvatii.m caiva tato jayamudiiyaret || 1 ||"),
        // Indic
        (Devanagari, "नारायणं नमस्कृत्य नरं चैव नरोत्तमम् । देवीं सरस्वतीं चैव ततो जयमुदीयरेत् ॥ १ ॥"),
        (Brahmi, "𑀦𑀸𑀭𑀸𑀬𑀡𑀁 𑀦𑀫𑀲𑁆𑀓𑀾𑀢𑁆𑀬 𑀦𑀭𑀁 𑀘𑁃𑀯 𑀦𑀭𑁄𑀢𑁆𑀢𑀫𑀫𑁆 𑁇 𑀤𑁂𑀯𑀻𑀁 𑀲𑀭𑀲𑁆𑀯𑀢𑀻𑀁 𑀘𑁃𑀯 𑀢𑀢𑁄 𑀚𑀬𑀫𑀼𑀤𑀻𑀬𑀭𑁂𑀢𑁆 𑁈 𑁧 𑁈"),
        (Grantha, "𑌨𑌾𑌰𑌾𑌯𑌣𑌂 𑌨𑌮𑌸𑍍𑌕𑍃𑌤𑍍𑌯 𑌨𑌰𑌂 𑌚𑍈𑌵 𑌨𑌰𑍋𑌤𑍍𑌤𑌮𑌮𑍍 । 𑌦𑍇𑌵𑍀𑌂 𑌸𑌰𑌸𑍍𑌵𑌤𑍀𑌂 𑌚𑍈𑌵 𑌤𑌤𑍋 𑌜𑌯𑌮𑍁𑌦𑍀𑌯𑌰𑍇𑌤𑍍 ॥ ௧ ॥"),
        (Gujarati, "નારાયણં નમસ્કૃત્ય નરં ચૈવ નરોત્તમમ્ । દેવીં સરસ્વતીં ચૈવ તતો જયમુદીયરેત્ ॥ ૧ ॥"),
        (Kannada, "ನಾರಾಯಣಂ ನಮಸ್ಕೃತ್ಯ ನರಂ ಚೈವ ನರೋತ್ತಮಮ್ । ದೇವೀಂ ಸರಸ್ವತೀಂ ಚೈವ ತತೋ ಜಯಮುದೀಯರೇತ್ ॥ ೧ ॥"),
        (Malayalam, "നാരായണം നമസ്കൃത്യ നരം ചൈവ നരോത്തമമ് । ദേവീം സരസ്വതീം ചൈവ തതോ ജയമുദീയരേത് ॥ ൧ ॥"),
        (Oriya, "ନାରାଯଣଂ ନମସ୍କୃତ୍ଯ ନରଂ ଚୈଵ ନରୋତ୍ତମମ୍ । ଦେଵୀଂ ସରସ୍ଵତୀଂ ଚୈଵ ତତୋ ଜଯମୁଦୀଯରେତ୍ ॥ ୧ ॥"),
        (Telugu, "నారాయణం నమస్కృత్య నరం చైవ నరోత్తమమ్ । దేవీం సరస్వతీం చైవ తతో జయముదీయరేత్ ॥ ౧ ॥"),
    ]);
}
