// I'm about to do something stupid

use std::fs::File;
use std::io::{self, BufRead};
use std::collections::{HashSet, HashMap};
fn main() {
    /*
     * 7 \ 1 gives top segment
     * Knowing that 2 is the only number that doesn't use the bottom right segment, we can deduce which letter is the bottom right using 2
     * Knowing the bottom right segment means we can use 1 to determine the top right segment
     * Removing 1, 7, 2, and 4 from the mix, we can conclude using 0 what the middle segment is since it's the only one that doesn't have the middle segment
     * Using 3, we can know what the bottom segment is
     * Using 9, we can figure out the top left segment and bottom left segment
     */

    let zero_segments = HashSet::from([&'a', &'b', &'c', &'e', &'f', &'g']);
    let one_segments = HashSet::from([&'c', &'f']);
    let two_segments = HashSet::from([&'a', &'c', &'d', &'e', &'g']);
    let three_segments = HashSet::from([&'a', &'c', &'d', &'f', &'g']);
    let four_segments = HashSet::from([&'b', &'c', &'d', &'f']);
    let five_segments = HashSet::from([&'a', &'b', &'d', &'f', &'g']);
    let six_segments = HashSet::from([&'a', &'b', &'d', &'e', &'f', &'g']);
    let seven_segments = HashSet::from([&'a', &'c', &'f']);
    let eight_segments = HashSet::from([&'a', &'b', &'c', &'d', &'e', &'f', &'g']);
    let nine_segments = HashSet::from([&'a', &'b', &'c', &'d', &'f', &'g']);

    let segments = [zero_segments, one_segments, two_segments, three_segments, four_segments, five_segments, six_segments, seven_segments, eight_segments, nine_segments];

    let input_file = File::open("input").expect("File not found");
    let input_lines:Vec<String> = io::BufReader::new(input_file).lines().map(|x| x.unwrap()).collect();

    let mut sum = 0;
    for line in input_lines{
        let mut segment_sets = Vec::new();
        let mut numbers: [Option<usize>; 10] = [None; 10];
        let line_components: Vec<&str> = line.split(" | ").collect();

        let signal_patterns = line_components[0].split(" ");
        let output_values = line_components[1].split(" ");

        let mut signal_mappings = HashMap::new();

        for signals in signal_patterns {
            let mut segment_set = HashSet::new();
            for segment in signals.chars(){
                segment_set.insert(segment);
            }
            match signals.len() {
                2 => {
                    numbers[1] = Some(segment_sets.len());
                },
                4 => {
                    numbers[4] = Some(segment_sets.len());
                },
                3 => {
                    numbers[7] = Some(segment_sets.len());
                },
                7 => {
                    numbers[8] = Some(segment_sets.len());
                }
                _ => {}
            }
            segment_sets.push(segment_set);
        }

        // find top segment
        let top_segment = segment_sets[numbers[7].unwrap()].difference(&segment_sets[numbers[1].unwrap()]).next().unwrap();
        signal_mappings.insert(top_segment, 'a');

        // bottom right segment

        let mut segment_counts = HashMap::new();

        for segment_set in &segment_sets {
            let inverted_segment = segment_sets[numbers[8].unwrap()].difference(&segment_set);
            for segment in inverted_segment {
                let current_count = *segment_counts.get(segment).unwrap_or(&0);
                segment_counts.insert(segment, current_count + 1);
            }
        }

        // count the inverted segment counts, take the segment that only has a count of 1
        let bottom_right_segment: &char = *segment_counts.iter().filter(|(k, v)| **v == 1).map(|(k, v)| k).next().unwrap();

        signal_mappings.insert(bottom_right_segment, 'f');

        // find top right segment

        for (i, segment_set) in segment_sets.iter().enumerate() {
            if !segment_set.contains(bottom_right_segment) {
                numbers[2] = Some(i);
            }
        }

        let top_right_segment = segment_sets[numbers[1].unwrap()].iter().filter(|x| x != &bottom_right_segment).next().unwrap();

        signal_mappings.insert(top_right_segment, 'c');

        // find top left segment

        let top_left_segment: &char = segment_sets[numbers[8].unwrap()].difference(&segment_sets[numbers[2].unwrap()]).into_iter().filter(|x| x != &bottom_right_segment).next().unwrap();

        signal_mappings.insert(top_left_segment, 'b');

    
        // find middle segment

        let cookie_cutter = HashSet::from([*top_left_segment, *top_right_segment, *bottom_right_segment]);

        let middle_segment = segment_sets[numbers[4].unwrap()].difference(&cookie_cutter).next().unwrap();

        signal_mappings.insert(middle_segment, 'd');


        // find bottom segment

        let cookie_cutter = HashSet::from([*top_left_segment, *top_segment, *top_right_segment, *middle_segment, *bottom_right_segment]);

        let (nine_index, nine_segments) = segment_sets.iter().enumerate().filter(|(_i, x)| x.is_superset(&cookie_cutter) && *_i != numbers[8].unwrap()).next().unwrap();

        numbers[9] = Some(nine_index);

        let bottom_segment = nine_segments.difference(&cookie_cutter).next().unwrap();

        signal_mappings.insert(bottom_segment, 'g');

        // find bottom left segment

        let bottom_left_segment = segment_sets[numbers[8].unwrap()].difference(&segment_sets[numbers[9].unwrap()]).into_iter().next().unwrap();

        signal_mappings.insert(bottom_left_segment, 'e');
        let mut display: usize = 0;
        for output_value in output_values {

            display *= 10;

            let translated_value: HashSet<&char> = output_value.chars().map(|x| signal_mappings.get(&x).unwrap()).collect();

            for (i, segment) in (&segments).iter().enumerate() {
                if translated_value == *segment {
                    display += i;
                    break;
                }
            }
        }
        sum += display;
    }


    println!("{}", sum);
}
