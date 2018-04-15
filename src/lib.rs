pub fn match_one(pattern:&Vec<&str>,text:&Vec<&str>) -> bool {
		if pattern.is_empty() {
			return true
		}
		if text.is_empty() {
			return false
		}

		if pattern[0] == "." || pattern[0] == text[0] {
			return true
		}else{
			return false
		}
}

pub fn search(mut pattern:Vec<&str>,text:Vec<&str>) -> bool {
	if pattern[0] == "^" {
		pattern.remove(0);
		return matchs(pattern,text)
	}else{
		pattern.insert(0,"*");
		pattern.insert(0,".");
		return matchs(pattern,text);
	}
}

pub fn matchs(mut pattern:Vec<&str>,mut text:Vec<&str>) -> bool {
	println!("{:?},{:?}",pattern,text);
	if pattern.is_empty() {
		return true
	}else if pattern.len() >= 1 && pattern[0] == "^" {
		pattern.remove(0);
		matchs(pattern,text)
	}else if text.is_empty() && pattern.len() >= 1 && pattern[0] == "$" {
		return true
	}else if pattern.len() >= 2 && pattern[1] == "*" {
			return match_star(pattern,text)
	}else if pattern.len() >= 2 && pattern[1] == "?" {
		return match_question(pattern,text)
	}else if match_one(&pattern,&text) {
			pattern.remove(0);
			text.remove(0);
			return matchs(pattern,text)
		}else{
			false
		}
}

pub fn match_star(pattern:Vec<&str>,text:Vec<&str>) -> bool {
	if match_one(&pattern,&text)
	&& matchs(pattern.clone(),text.clone()[1..text.len()].to_vec())
	|| matchs(pattern[2..pattern.len()].to_vec(),text) {
		true
	}else{
		false
	}
}
pub fn match_question(pattern:Vec<&str>,text:Vec<&str>) -> bool {
	if match_one(&pattern,&text)
	&& matchs(pattern.clone()[2..pattern.len()].to_vec(),text.clone()[1..pattern.len()].to_vec())
	|| matchs(pattern.clone()[2..pattern.len()].to_vec(),text) {
		true
	}else{
		false
	}
}
