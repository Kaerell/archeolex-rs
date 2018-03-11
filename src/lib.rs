#![allow(dead_code)]
extern crate chrono;
extern crate chrono_tz;


#[derive(Debug)]
// TODO: should accept gregorian and republican
enum CodeDate {
	Some(chrono::DateTime<chrono::Utc>),
	None,
	Unknown,
	Invalid(String),
}

enum Nature {
	Constitution,
	ConstitutionalLaw,
	OrganicLaw,
	Law,
	LegalCode,
	LawDecree,
	Decree,
	Convention,
	Order,
	Memo,
	Decision,
	Statement,
}

enum Base {
	LEGI,
	JORF,
	KALI,
	CNIL,
	CONSTIT
}

enum LegalState {
	InEffect,
	InEffectDiff,
	Amended,
	AmendedStillBorn,
	Abrogated,
	AbrogatedDiff,
	Canceled,
	Outdated,
	Transfered,
}


struct Article {
	name: &'static str,
	legal_state: LegalState,
	issue: usize,
	start_date: CodeDate,
	end_date: CodeDate,
	text: LegalText,
	section_ver: Box<SectionVer>,
	text_ver: LegalTextVer<Base>,
}

struct LegalText {
	//cid_id: [u8; 20], FR uniquement
	decree_id: usize,
	nature: Nature,
	publication_date: CodeDate,
	enactment_date: CodeDate,
}

struct LegalTextVer<Base> {
	text: LegalText,
	title: &'static str,
	full_title: &'static str,
	legal_state: LegalState,
	start_date: CodeDate,
	end_date: CodeDate,
	base: Base,
}

struct Section {
	//cid_id: [u8; 20], FR uniquement
	parent_id: usize,
	level: usize,
	text: LegalText,
}

struct SectionVer {
	//cid_id: [u8; 20], FR uniquement
	parent: Box <SectionVer>,
	name: &'static str,
	legal_state: LegalState,
	level: usize,
	issue: usize,
	start_date: CodeDate,
	end_date: CodeDate,
	text: LegalText,
	text_version: LegalTextVer<Base>,
}


#[cfg(test)]
mod tests {
	#[test]
	fn test_list_revisions() {
		// let ref_code;
		// let ref_article;


		// assert_eq!(ref_code.list_revisions(ref_article).len(), 8);
	}
	#[test]
	fn test_diff() {
		// let ref_code;
		// let ref_article;


		// assert_eq!(ref_code.diff_dates(ref_article, date_1, date_2), "+ bonjour");
		// assert_eq!(ref_code.diff(ref_article, id_1, id_2), "+ bonjour");
	}
	#[test]
	fn test_download_texts() {

	}

	#[test]
	fn test_export_git() {
		
	}

	#[test]
	fn test_export_markdown() {
		
	}

	#[test]
	fn test_reset() {
		
	}

	#[test]
	fn test_update() {
		
	}

	#[test]
	fn test_download_release() {
		
	}
}
