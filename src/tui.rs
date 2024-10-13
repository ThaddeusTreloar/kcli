
let mut terminal = ratatui::init();

let file = File::open("asd.json").unwrap();

    let buffered = BufReader::new(file);

    let lines = buffered
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()
        .unwrap();

    let content = lines.join("\n");
    let content_height = lines.len();

    let current_local = Local::now();

    let vertical_scroll = Cell::new(0); // from app state

    loop {
        let current_height = Cell::new(0);

        terminal
            .draw(|frame| {
                let mut vertical_scroll = vertical_scroll.get(); // from app state
                current_height.set(frame.area().height);

                let paragraph = Paragraph::new(content.clone())
                    .scroll((vertical_scroll as u16, 0))
                    .block(Block::new().borders(Borders::RIGHT)); // to show a background for the scrollbar

                let scrollbar = Scrollbar::new(ScrollbarOrientation::VerticalRight)
                    .begin_symbol(Some("↑"))
                    .end_symbol(Some("↓"));

                let scrollbar_height = if frame.area().height as usize > content_height {
                    frame.area().height as usize
                } else {
                    content_height - frame.area().height as usize
                };

                let mut scrollbar_state =
                    ScrollbarState::new(scrollbar_height).position(vertical_scroll);

                let area = frame.area();
                // Note we render the paragraph
                frame.render_widget(paragraph, area);
                // and the scrollbar, those are separate widgets
                frame.render_stateful_widget(
                    scrollbar,
                    area.inner(Margin {
                        // using an inner vertical margin of 1 unit makes the scrollbar inside the block
                        vertical: 1,
                        horizontal: 0,
                    }),
                    &mut scrollbar_state,
                );
            })
            .expect("Failed to draw");
        if let Event::Key(key) = event::read().expect("Failed to draw") {
            if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                break;
            } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Down {
                let mut val = vertical_scroll.get();

                if current_height.get() as usize > content_height {
                    continue;
                } else if val < (content_height - current_height.get() as usize) {
                    val += 1;

                    vertical_scroll.set(val);
                } else if val > (content_height - current_height.get() as usize) {
                    vertical_scroll.set(content_height - current_height.get() as usize);
                }
            } else if key.kind == KeyEventKind::Press && key.code == KeyCode::Up {
                let mut val = vertical_scroll.get();

                if val >= 1 {
                    val -= 1;

                    vertical_scroll.set(val);
                }
            }
        }
    }
    ratatui::restore();