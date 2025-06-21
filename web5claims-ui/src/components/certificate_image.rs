use crate::utils::{format_course_name, get_achievement_level};
use konnektoren_core::certificates::CertificateData;
use yew::prelude::*;

#[derive(Properties, PartialEq, Clone)]
pub struct Web5CertificateImageProps {
    pub certificate_data: CertificateData,
}

#[function_component(Web5CertificateImage)]
pub fn web5_certificate_image(props: &Web5CertificateImageProps) -> Html {
    let cert = &props.certificate_data;

    // Extract language and level from game_path_name
    let (language, level) = parse_course_info(&cert.game_path_name);
    let grade = get_grade_from_percentage(cert.performance_percentage);
    let achievement_level = get_achievement_level(cert.performance_percentage);
    let formatted_course = format_course_name(&cert.game_path_name);

    // Convert date to string
    let formatted_date = cert.date.format("%B %d, %Y").to_string();

    html! {
        <div class="relative w-full max-w-2xl mx-auto">
            // Main Certificate Card
            <div class="bg-gradient-to-br from-white via-blue-50 to-indigo-100 border-4 border-blue-600 rounded-lg shadow-2xl overflow-hidden">
                // Header with Logo and Title
                <div class="bg-gradient-to-r from-blue-600 to-indigo-700 text-white p-6 text-center">
                    <div class="flex items-center justify-center mb-2">
                        <div class="text-3xl mr-3">{"🔐"}</div>
                        <h1 class="text-2xl font-bold">{"Web5 Claims"}</h1>
                    </div>
                    <p class="text-blue-100 text-sm">{"Zero-Knowledge Language Certification"}</p>
                </div>

                // Certificate Content
                <div class="p-8 text-center">
                    <h2 class="text-3xl font-serif text-gray-800 mb-6">
                        {"Certificate of Achievement"}
                    </h2>

                    <div class="mb-6">
                        <p class="text-lg text-gray-600 mb-2">{"This certifies that"}</p>
                        <p class="text-2xl font-bold text-blue-700 mb-4">{&cert.profile_name}</p>
                        <p class="text-lg text-gray-600">{"has successfully completed the"}</p>
                    </div>

                    // Course Information
                    <div class="bg-blue-50 border border-blue-200 rounded-lg p-6 mb-6">
                        <div class="flex justify-center items-center mb-4">
                            <span class="text-4xl mr-3">{get_flag_emoji(&language)}</span>
                            <div class="text-left">
                                <h3 class="text-xl font-bold text-gray-800">{formatted_course}</h3>
                                <p class="text-gray-600">{"Language Proficiency Course"}</p>
                                <p class="text-sm text-blue-600 font-semibold">{achievement_level}</p>
                            </div>
                        </div>

                        // Performance Stats
                        <div class="grid grid-cols-3 gap-4 text-center">
                            <div>
                                <div class="text-2xl font-bold text-blue-600">{cert.performance_percentage}{"%"}</div>
                                <div class="text-sm text-gray-600">{"Performance"}</div>
                            </div>
                            <div>
                                <div class="text-2xl font-bold text-green-600">{cert.solved_challenges}</div>
                                <div class="text-sm text-gray-600">{"Completed"}</div>
                            </div>
                            <div>
                                <div class="text-2xl font-bold text-purple-600">{grade}</div>
                                <div class="text-sm text-gray-600">{"Grade"}</div>
                            </div>
                        </div>
                    </div>

                    // Date and Verification
                    <div class="flex justify-between items-center text-sm text-gray-600 mb-4">
                        <div>
                            <p>{"Issued on"}</p>
                            <p class="font-semibold">{formatted_date}</p>
                        </div>
                        <div class="text-center">
                            <div class="w-16 h-16 bg-blue-600 rounded-full flex items-center justify-center text-white text-xl mb-1">
                                {"🎓"}
                            </div>
                            <p class="text-xs">{"Verified"}</p>
                        </div>
                        <div class="text-right">
                            <p>{"ZK Verification"}</p>
                            <p class="font-semibold text-green-600">{"Available"}</p>
                        </div>
                    </div>

                    // ZK Badge and Security Features
                    <div class="border-t border-gray-300 pt-4">
                        <div class="flex justify-center items-center space-x-2 text-xs text-gray-500 mb-2">
                            <span class="bg-gradient-to-r from-purple-500 to-blue-500 text-white px-3 py-1 rounded-full">
                                {"🔐 ZK-Verifiable"}
                            </span>
                            <span class="bg-gray-100 px-2 py-1 rounded">{"Powered by Aleo"}</span>
                            <span class="bg-yellow-100 px-2 py-1 rounded">{"ZK Hack Berlin 2024"}</span>
                        </div>

                        // Certificate ID (first 8 chars of base64)
                        <div class="text-xs text-gray-400 font-mono">
                            {"Certificate ID: "}{&cert.to_base64()[..8]}{"..."}
                        </div>
                    </div>
                </div>
            </div>

            // Decorative corner elements
            <div class="absolute -top-2 -left-2 w-6 h-6 bg-yellow-400 rounded-full shadow-lg"></div>
            <div class="absolute -top-2 -right-2 w-6 h-6 bg-yellow-400 rounded-full shadow-lg"></div>
            <div class="absolute -bottom-2 -left-2 w-6 h-6 bg-yellow-400 rounded-full shadow-lg"></div>
            <div class="absolute -bottom-2 -right-2 w-6 h-6 bg-yellow-400 rounded-full shadow-lg"></div>
        </div>
    }
}

fn parse_course_info(game_path_name: &str) -> (String, String) {
    let parts: Vec<&str> = game_path_name.split('_').collect();
    if parts.len() >= 2 {
        let language = parts[0].to_string();
        let level = parts[1].to_string();
        (language, level)
    } else {
        ("Language".to_string(), "Course".to_string())
    }
}

fn get_flag_emoji(language: &str) -> &'static str {
    match language.to_lowercase().as_str() {
        "german" => "🇩🇪",
        "spanish" => "🇪🇸",
        "french" => "🇫🇷",
        "italian" => "🇮🇹",
        "english" => "🇺🇸",
        "portuguese" => "🇵🇹",
        "dutch" => "🇳🇱",
        "russian" => "🇷🇺",
        "chinese" => "🇨🇳",
        "japanese" => "🇯🇵",
        _ => "🌍",
    }
}

fn get_grade_from_percentage(percentage: u8) -> &'static str {
    match percentage {
        95..=100 => "A+",
        90..=94 => "A",
        85..=89 => "B+",
        80..=84 => "B",
        75..=79 => "C+",
        70..=74 => "C",
        65..=69 => "D+",
        60..=64 => "D",
        _ => "F",
    }
}
