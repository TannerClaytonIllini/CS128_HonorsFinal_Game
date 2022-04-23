use coffee::graphics::{
    Color, Frame, HorizontalAlignment, Window, WindowSettings,
};
use coffee::load::Task;

use coffee::ui::{
    button, slider, Align, Button, Checkbox, Column, Element, Justify, Radio,
    Renderer, Row, Slider, Text, UserInterface,
};
use coffee::input::{self, gamepad, Input};
use coffee::{Game, Result, Timer};

pub fn main() -> Result<()> {
    <Tour as UserInterface>::run(WindowSettings {
        title: String::from("BalckJack! by Everything is exploding"),
        size: (1280, 1024),
        resizable: true,
        fullscreen: false,
        maximized: false,
    })
}

struct Tour {
    steps: Steps,
    back_button: button::State,
    next_button: button::State,
    
}

impl Game for Tour {
    type Input = ();
    type LoadingScreen = ();

    fn load(_window: &Window) -> Task<Tour> {
        Task::succeed(|| Tour {
            steps: Steps::new(),
            back_button: button::State::new(),
            next_button: button::State::new(),
        })
    }

    fn draw(&mut self, frame: &mut Frame, _timer: &Timer) {
        frame.clear(Color {
            r: 0.3,
            g: 0.3,
            b: 0.6,
            a: 1.0,
        });
    }
}

impl UserInterface for Tour {
    type Message = Message;
    type Renderer = Renderer;

    fn react(&mut self, event: Message, _window: &mut Window) {
        match event {
            Message::BackPressed => {
                self.steps.go_back();
            }
            Message::NextPressed => {
                self.steps.advance();
            }
            Message::StepMessage(step_msg) => {
                self.steps.update(step_msg);
        }
    }
    }
    fn layout(&mut self, window: &Window) -> Element<Message> {
        let Tour {
            steps,
            back_button,
            next_button,
        } = self;
        let mut controls = Row::new();
        if steps.has_previous() {
            controls = controls.push(
                Button::new(back_button, "Back")
                .on_press(Message::BackPressed)
                .class(button::Class::Secondary),
            );
        }
        controls = controls.push(Column::new());
        if steps.can_continue() {
            controls = controls.push(
                Button::new(next_button, "Next").on_press(Message::NextPressed),
            );
        }

        let content = Column::new()
            .max_width(500)
            .spacing(20)
            .push(steps.layout().map(Message::StepMessage))
            .push(controls);

        Column::new()
            .width(window.width() as u32)
            .height(window.height() as u32)
            .padding(20)
            .align_items(Align::Center)
            .justify_content(Justify::Center)
            .push(content)
            .into()
    }

}
#[derive(Debug, Clone, Copy)]
enum Message {
    BackPressed,
    NextPressed,
    StepMessage(StepMessage),
}

struct Steps {
    steps: Vec<Step>,
    current: usize,
}

impl Steps {
    fn new() -> Steps {
        Steps {
            steps: vec![
                Step::Welcome,
                Step::Buttons {
                    primary: button::State::new(),
                    secondary: button::State::new(),
                    positive: button::State::new(),
                },
                Step::Checkbox { is_checked: false },
                Step::End{
                    primary: button::State::new(),
                    secondary: button::State::new(),
                },
            ],
            current: 0,
        }
    }

    fn update(&mut self, msg: StepMessage) {
        self.steps[self.current].update(msg);
    }

    fn layout(&mut self) -> Element<StepMessage> {
        self.steps[self.current].layout()
    }

    fn advance(&mut self) {
        if self.can_continue() {
            self.current += 1;
        }
    }

    fn go_back(&mut self) {
        if self.has_previous() {
            self.current -= 1;
        }
    }

    fn has_previous(&self) -> bool {
        self.current > 0
    }

    fn can_continue(&self) -> bool {
        self.current + 1 < self.steps.len()
            && self.steps[self.current].can_continue()
    }
}

enum Step {
    Welcome,
    Buttons {
        primary: button::State,
        secondary: button::State,
        positive: button::State,
    },
    Checkbox {
        is_checked: bool,
    },
    End{
        primary: button::State,
        secondary: button::State,
     
    },
}

#[derive(Debug, Clone, Copy)]
enum StepMessage {
    CheckboxToggled(bool),
}

impl<'a> Step {
    fn update(&mut self, msg: StepMessage) {
        match msg {
            StepMessage::CheckboxToggled(value) => {
                if let Step::Checkbox { is_checked } = self {
                    *is_checked = value;
                }
            }
        };
    }
    
    fn can_continue(&self) -> bool {
        match self {
            Step::Welcome => true,
            Step::Buttons { .. } => true,
            Step::Checkbox { is_checked } => *is_checked,
            Step::End{..} => false,
        }
    }
    fn layout(&mut self) -> Element<StepMessage> {
        match self {
            Step::Welcome => Self::welcome().into(),
            Step::Buttons {
                primary,
                secondary,
                positive,
            } => Self::buttons(primary, secondary, positive).into(),
            Step::Checkbox { is_checked } => Self::checkbox(*is_checked).into(),
            Step::End {
                primary,
                secondary,
            }=> Self::end(primary, secondary).into(),
        }
    }

    fn container(title: &str) -> Column<'a, StepMessage> {
        Column::new()
            .spacing(20)
            .align_items(Align::Stretch)
            .push(Text::new(title).size(50))
    }
    fn welcome() -> Column<'a, StepMessage> {
        Self::container("Welcome to Blackjack!")
            .push(Text::new(
                "This game assumes you are already are familiar with Blackjack and currently doesn't explain how play.
                If that doesn't work for you, kindly Combust.",
            ))
            .push(Text::new(
                "Have a Great Game!",
            ))
    }
    fn buttons(
        primary: &'a mut button::State,
        secondary: &'a mut button::State,
        positive: &'a mut button::State,
    ) -> Column<'a, StepMessage> {
        Self::container("Dealer")
            .push(Text::new("one dealer's hand visible"))
            .push(Text::new("one dealer's hand invisible"))
            .push(Button::new(primary, "Hit"))
            .push(
                Button::new(secondary, "Pass")
                    .class(button::Class::Secondary),
            )
            
            .push(Text::new(
                "Your hand",
            ))
    }
    fn checkbox(is_checked: bool) -> Column<'a, StepMessage> {
        Self::container("Test (for developing)")
            .push(Text::new(
                "A box that can be checked. Useful to build toggle controls.",
            ))
            .push(Checkbox::new(
                is_checked,
                "Show \"Next\" button",
                StepMessage::CheckboxToggled,
            ))
            .push(Text::new(
                "A checkbox always has a label, and both the checkbox and its \
                 label can be clicked to toggle it.",
            ))
    }
    fn end(
        primary: &'a mut button::State,
        secondary: &'a mut button::State,
    ) -> Column<'a, StepMessage> {
        Self::container("You reached the end!")
            .push(Text::new(
                "This tour will be extended as more features are added.",
            ))
            .push(Button::new(primary, "restart"))
            .push(Button::new(secondary, "end"))
    }
}

