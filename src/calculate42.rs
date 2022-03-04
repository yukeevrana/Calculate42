use colored::Colorize;

pub struct Calc {
    output_color: String,
    input: String,
    output: String
}

impl Calc {
    pub fn new() -> Self {
        Calc {
            output_color: String::from("white"),
            input: String::new(),
            output: String::new()
        }
    }

    pub fn take_message(&mut self, message: String) -> Result<colored::ColoredString, &str> {
        self.input = message.clone();

        if self.input.as_str() == "exit" { return Err("exit"); }
        
        self.find_color_command();

        match self.find_math_expr() {
            Err(_) => {},
            Ok(_) => {}
        };

        Ok(self.get_answer())
    }

    fn find_color_command(&mut self) {
        let color = self.input.as_str();
        if color == "black" || color == "red" || color == "green" || color == "yellow" || color == "blue" || 
            color == "magenta" || color == "cyan" || color == "white" || color == "bright black" || 
            color == "bright red" || color == "bright green" || color == "bright yellow" || color == "bright blue" || 
            color == "bright magenta" || color == "bright cyan" || color == "bright white" {

            self.output_color = String::from(color);
            self.output = String::from("Done");
        }
    }

    fn find_math_expr(&mut self) -> Result<String, &str> {
        let mut res = String::new();
        let mut st = Vec::new();

        for ch in self.input.chars() {
            match ch {
                // "(" => st.push(grapheme),
                // ")" => {
                //     while st.last() != Some(&"(") {
                //         res.push(st.pop().unwrap());
                //     } 
                //     st.pop();
                // },
                '+' => {
                    if st.last() == Some(&'+') {
                        res.push(st.pop().unwrap());
                    }
                    st.push(ch);
                }
                number => {
                    let n = ch.to_digit(10);
                    match n {
                        None => { return Err("Parsing error"); },
                        _ => { res.push(number); }
                    }
                }
            }
        }

        for ch in st.iter().rev() {
            res.push(*ch);
        }

        self.input = res;

        Ok(String::from(""))
    }

    fn get_answer(&self) -> colored::ColoredString {
        match self.output_color.as_str() {
            "blue"              => self.output.blue(),
            "green"             => self.output.green(),
            "red"               => self.output.red(),
            "black"             => self.output.black(), 
            "yellow"            => self.output.yellow(), 
            "magenta"           => self.output.magenta(), 
            "cyan"              => self.output.cyan(), 
            "bright black"      => self.output.bright_black(), 
            "bright red"        => self.output.bright_red(),
            "bright green"      => self.output.bright_green(), 
            "bright yellow"     => self.output.bright_yellow(), 
            "bright blue"       => self.output.bright_blue(), 
            "bright magenta"    => self.output.bright_magenta(),
            "bright cyan"       => self.output.bright_cyan(), 
            "bright white"      => self.output.bright_white(),
            _                   => self.output.white()
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn find_color_command_correct_color() {

        for color in [ "black", "red", "green", "yellow", "blue",
                    "magenta", "cyan", "white", "bright black", "bright red",
                    "bright green", "bright yellow", "bright blue", "bright magenta",
                    "bright cyan", "bright white" ] {

            let mut calc = super::Calc::new();
            assert_eq!(String::from("white"), calc.output_color);

            calc.input = String::from(color);
            calc.find_color_command();

            assert_eq!(String::from(color), calc.output_color);
            assert_eq!(String::from("Done"), calc.output);
        }
    }

    #[test]
    fn find_color_command_incorrect_color() {

        for color in [ "not a color at all", "rose", "purple", "exit", "another command" ] {

            let mut calc = super::Calc::new();
            assert_eq!(String::from("white"), calc.output_color);

            calc.input = String::from(color);
            calc.find_color_command();

            assert_eq!(String::from("white"), calc.output_color);
            assert_eq!(String::new(), calc.output);
        }
    }

    #[test]
    fn get_answer_with_correct_color() {
        use colored::Colorize;

        let mut calc = super::Calc::new();
        assert_eq!(String::from("white"), calc.output_color);

        calc.output = String::from("Some shiny output");
        assert_eq!(String::from("Some shiny output").white(), calc.get_answer());

        calc.output_color = String::from("black");
        assert_eq!(String::from("Some shiny output").black(), calc.get_answer());

        calc.output_color = String::from("green");
        assert_eq!(String::from("Some shiny output").green(), calc.get_answer());

        calc.output_color = String::from("yellow");
        assert_eq!(String::from("Some shiny output").yellow(), calc.get_answer());

        calc.output_color = String::from("red");
        assert_eq!(String::from("Some shiny output").red(), calc.get_answer());

        calc.output_color = String::from("blue");
        assert_eq!(String::from("Some shiny output").blue(), calc.get_answer());

        calc.output_color = String::from("magenta");
        assert_eq!(String::from("Some shiny output").magenta(), calc.get_answer());

        calc.output_color = String::from("cyan");
        assert_eq!(String::from("Some shiny output").cyan(), calc.get_answer());

        calc.output_color = String::from("bright white");
        assert_eq!(String::from("Some shiny output").bright_white(), calc.get_answer());

        calc.output_color = String::from("bright black");
        assert_eq!(String::from("Some shiny output").bright_black(), calc.get_answer());

        calc.output_color = String::from("bright green");
        assert_eq!(String::from("Some shiny output").bright_green(), calc.get_answer());

        calc.output_color = String::from("bright yellow");
        assert_eq!(String::from("Some shiny output").bright_yellow(), calc.get_answer());

        calc.output_color = String::from("bright red");
        assert_eq!(String::from("Some shiny output").bright_red(), calc.get_answer());

        calc.output_color = String::from("bright blue");
        assert_eq!(String::from("Some shiny output").bright_blue(), calc.get_answer());

        calc.output_color = String::from("bright magenta");
        assert_eq!(String::from("Some shiny output").bright_magenta(), calc.get_answer());

        calc.output_color = String::from("bright cyan");
        assert_eq!(String::from("Some shiny output").bright_cyan(), calc.get_answer());
    }

    #[test]
    fn get_answer_with_incorrect_color() {
        use colored::Colorize;

        let mut calc = super::Calc::new();
        assert_eq!(String::from("white"), calc.output_color);

        calc.output = String::from("Some shiny output");
        assert_eq!(String::from("Some shiny output").white(), calc.get_answer());

        calc.output_color = String::from("incorrect color");
        assert_eq!(String::from("Some shiny output").white(), calc.get_answer());
    }
}
