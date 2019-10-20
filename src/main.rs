use rand::seq::SliceRandom;
use rand::thread_rng;
use rand::Rng;
use std::borrow::Borrow;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::Write;
#[macro_use]
extern crate serde;
extern crate serde_xml_rs;
#[derive(Deserialize, Debug, Default, Clone)]
struct Toml {
    config: Config,
}
#[derive(Deserialize, Debug, Default, Clone)]
struct Config {
    path: String,
    factions: Vec<String>,
    types: Vec<String>,
    products: Vec<String>,
}

fn main() {
    let mut toml_str = include_str!("Config.toml");
    let mut toml_parsed: Toml = toml::from_str(&toml_str).unwrap_or_default();
    let mut god = fs::read_to_string(&toml_parsed.config.path).unwrap();
//                 god.push_str("<?xml version=\"1.0\" encoding=\"utf-8\" ?>\n
// <diff>\n
//     <add sel=\"/jobs\">\n");
    let mut name_number = 0;
    let runs = 3;

    for _run in 0..runs {
        for faction in toml_parsed.config.factions.iter() {
            name_number += 1;
            let chosen_type = &toml_parsed.config.types.choose(&mut rand::thread_rng()).expect("weapontype select failed");
            let chosen_prod = &toml_parsed.config.products.choose(&mut rand::thread_rng()).expect("weapontype select failed");
            god.push_str(&format!(
                "<station id=\"station_{}_{}\" race=\"argon\" owner=\"{}\" >
              <quotas>
                <quota galaxy=\"1\" />
              </quotas>
               <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"[ownerless, {}]\" />
              <station>
                <select faction=\"{}\" tags=\"[{}]\" />
                <loadout>
                  <level exact=\"0.75\" />
                </loadout>
              </station>
            </station>\n",
                name_number, faction, faction,faction, "argon", chosen_type
            ));
    //         god.push_str(&format!(
    //             "<product id=\"prod_{}_{}\" ware=\"{}\" owner=\"{}\" type=\"factory\">
    //   <quotas>
    //     <quota galaxy=\"1\" />
    //   </quotas>
    // //   <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"[ownerless,argon, hatikvah, teladi, paranid, holyorder, antigone]\" relation=\"self\" comparison=\"ge\"/>
    //   <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"[ownerless]\" relation=\"self\" comparison=\"ge\"/>
    //   <module>
    //     <select ware=\"{}\" race=\"argon\" />
    //   </module>
    // </product>\n",
    //             name_number, faction, chosen_prod, faction, chosen_prod
    //         ));

  //           god.push_str(&format!(
  //               "<job id=\"frigate_patrol_{}\"  >
  //   <modifiers commandeerable=\"true\"/>
  //   <orders>
  //     <order order=\"Patrol\" default=\"true\">
  //       <param name=\"range\" value=\"class.sector\"/>
  //     </order>
  //   </orders>
  //   <category faction=\"argon\" tags=\"[military, frigate]\" size=\"ship_m\"/>
  //   <quota galaxy=\"3\" sector=\"3\"/>
  //   <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"{}\" relation=\"neutral\" comparison=\"ge\"/>
  //   <environment buildatshipyard=\"true\"/>
  //   <ship>
  //     <select faction=\"argon\" tags=\"[military, frigate]\" size=\"ship_m\"/>
  //     <loadout>
  //       <level min=\"0.4\" max=\"1.0\"/>
  //     </loadout>
  //     <owner exact=\"{}\" overridenpc=\"true\"/>
  //   </ship>
  //   <subordinates>
  //     <subordinate job=\"argon_fighter_escort_s_frigate\"/>
  //   </subordinates>
  // </job>\n",
  //               faction, faction, faction
  //           ));
  //           god.push_str(&format!(
  //               "<job id=\"destroyer_{}\" >
  //   <modifiers commandeerable=\"true\"/>
  //   <orders>
  //     <order order=\"Patrol\" default=\"true\">
  //       <param name=\"range\" value=\"class.cluster\"/>
  //     </order>
  //   </orders>
  //   <category faction=\"argon\" tags=\"[military, destroyer]\" size=\"ship_l\"/>
  //   <quota galaxy=\"4\" sector=\"1\"/>
  //   <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"{}\" relation=\"neutral\" comparison=\"ge\"/>
  //   <environment buildatshipyard=\"true\"/>
  //   <ship>
  //     <select faction=\"argon\" tags=\"[military, destroyer]\" size=\"ship_l\"/>
  //     <loadout>
  //       <level exact=\"1.0\"/>
  //       <variation exact=\"0\"/>
  //     </loadout>
  //     <owner exact=\"{}\" overridenpc=\"true\"/>
  //   </ship>
  //   <subordinates>
  //     <subordinate job=\"argon_frigate_escort_m\"/>
  //   </subordinates>
  // </job>\n",
  //               faction, faction, faction
  //           ));
  //           god.push_str(&format!(
  //               " <job id=\"carrier_{}\" >
  //   <modifiers commandeerable=\"true\"/>
  //   <orders>
  //     <order order=\"Patrol\" default=\"true\">
  //       <param name=\"range\" value=\"class.zone\"/>
  //     </order>
  //   </orders>
  //   <category faction=\"argon\" tags=\"[military, carrier]\" size=\"ship_xl\"/>
  //   <quota galaxy=\"1\"/>
  //   <!--<quota galaxy=\"14\" cluster=\"1\"/>-->
  //   <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"{}\" relation=\"neutral\" comparison=\"exact\"/>
  //   <environment buildatshipyard=\"true\"/>
  //   <ship>
  //     <select faction=\"argon\" tags=\"[military, carrier]\" size=\"ship_xl\"/>
  //     <loadout>
  //       <level exact=\"1.0\"/>
  //       <variation exact=\"0\"/>
  //       <!--<level min=\"0.7\" max=\"1.0\"/>-->
  //     </loadout>
  //     <owner exact=\"{}\" overridenpc=\"true\"/>
  //   </ship>
  //   <subordinates>
  //     <subordinate job=\"argon_destroyer_escort_l\"/>
  //     <subordinate job=\"argon_fighter_escort_s_carrier\"/>
  //     <subordinate job=\"argon_resupplier_escort_xl\"/>
  //   </subordinates>
  // </job>\n",
  //               faction, faction,faction
  //           ));
//             god.push_str(&format!(
//                 " <job id=\"free_miner_l_{}\" >
//     <orders>
//       <order order=\"MiningRoutine\" default=\"true\">
//         <param name=\"minbuy\" value=\"0\"/>
//         <param name=\"maxbuy\" value=\"5\"/>
//         <param name=\"minsell\" value=\"0\"/>
//         <param name=\"maxsell\" value=\"5\"/>
//       </order>
//     </orders>
//     <category faction=\"argon\" tags=\"[freighter, miner, deepspace]\" size=\"ship_l\"/>
//     <quota galaxy=\"3\"/>
//     <location class=\"galaxy\" macro=\"xu_ep2_universe_macro\" faction=\"{}\" relation=\"neutral\" comparison=\"ge\" regionbasket=\"minerals\"/>
//     <environment buildatshipyard=\"true\"/>
//     <basket basket=\"minerals\"/>
//     <encounters id=\"mining_group_large\"/>
//     <ship>
//       <select faction=\"argon\" tags=\"[miner, solid]\" size=\"ship_l\"/>
//       <loadout>
//         <level min=\"0\" max=\"0.7\"/>
//       </loadout>
//       <owner exact=\"{}\" overridenpc=\"true\"/>
//     </ship>
//     <subordinates>
//       <subordinate job=\"argon_miningfleet_ml_solid_large\"/>
//       <subordinate job=\"argon_fighter_escort_s_miningfleet\"/>
//     </subordinates>
//   </job>\n",
//                 faction, faction,faction
//             ));
        }
    }
//                 god.push_str("    \n</add>
// \n</diff>");
    let mut outputfile = File::create(&toml_parsed.config.path).expect("create file failed");
    outputfile.write_all(god.as_bytes()).unwrap();
}
// if chosen_type == &"wharf" { "factory"} else {chosen_type },

// FRIGATE PATROL
// <job id="argon_frigate_patrol_m_sector_exp" name="{20204,2901}" comment="each sector, expansion">
//     <modifiers commandeerable="false"/>
//     <orders>
//       <order order="Patrol" default="true">
//         <param name="range" value="class.sector"/>
//       </order>
//     </orders>
//     <category faction="argon" tags="[military, frigate]" size="ship_m"/>
//     <quota galaxy="55" sector="2"/>
//     <location class="galaxy" macro="xu_ep2_universe_macro" faction="argon" relation="self" comparison="exact"/>
//     <environment buildatshipyard="true"/>
//     <ship>
//       <select faction="argon" tags="[military, frigate]" size="ship_m"/>
//       <loadout>
//         <level min="0.4" max="1.0"/>
//       </loadout>
//       <owner exact="argon" overridenpc="true"/>
//     </ship>
//     <subordinates>
//       <subordinate job="argon_fighter_escort_s_frigate"/>
//     </subordinates>
//   </job>

//  <job id="paranid_destroyer_patrol_l_sector_exp" name="{20204,3301}" comment="each sector, expansion">
//     <modifiers commandeerable="true"/>
//     <orders>
//       <order order="Patrol" default="true">
//         <param name="range" value="class.cluster"/>
//       </order>
//     </orders>
//     <category faction="paranid" tags="[factionlogic, military, destroyer]" size="ship_l"/>
//     <quota galaxy="14" sector="1"/>
//     <location class="galaxy" macro="xu_ep2_universe_macro" faction="paranid" relation="self" comparison="exact"/>
//     <environment buildatshipyard="true"/>
//     <ship>
//       <select faction="paranid" tags="[military, destroyer]" size="ship_l"/>
//       <loadout>
//         <level exact="1.0"/>
//         <variation exact="0"/>
//       </loadout>
//       <owner exact="paranid" overridenpc="true"/>
//     </ship>
//     <subordinates>
//       <subordinate job="paranid_frigate_escort_m"/>
//     </subordinates>
//   </job>

// argon_frigate_escort_m
// argon_frigate_escort_m
// argon_frigate_escort_m
// antigone_frigate_escort_m
// antigone_frigate_escort_m
// antigone_frigate_escort_m
// paranid_frigate_escort_m
// paranid_frigate_escort_m
// paranid_frigate_escort_m
// holyorder_frigate_escort_m
// holyorder_frigate_escort_m
// holyorder_frigate_escort_m
// teladi_frigate_escort_m
// teladi_frigate_escort_m
// teladi_frigate_escort_m
// ministry_frigate_escort_m
// ministry_frigate_escort_m
// ministry_frigate_escort_m
// ministry_frigate_escort_m

// <job id="paranid_carrier_patrol_xl_cluster" name="{20204,3201}" comment="one">
//     <modifiers commandeerable="true"/>
//     <orders>
//       <order order="Patrol" default="true">
//         <param name="range" value="class.zone"/>
//       </order>
//     </orders>
//     <category faction="paranid" tags="[factionlogic, military, carrier]" size="ship_xl"/>
//     <quota galaxy="1"/>
//     <!--<quota galaxy="14" cluster="1"/>-->
//     <location class="galaxy" macro="xu_ep2_universe_macro" faction="paranid" relation="self" comparison="exact"/>
//     <environment buildatshipyard="true"/>
//     <ship>
//       <select faction="paranid" tags="[military, carrier]" size="ship_xl"/>
//       <loadout>
//         <level exact="1.0"/>
//         <variation exact="0"/>
//         <!--<level min="0.7" max="1.0"/>-->
//       </loadout>
//       <owner exact="paranid" overridenpc="true"/>
//     </ship>
//     <subordinates>
//       <subordinate job="paranid_destroyer_escort_l"/>
//       <subordinate job="paranid_fighter_escort_s_carrier"/>
//       <subordinate job="paranid_resupplier_escort_xl"/>
//     </subordinates>
//   </job>

// <job id="paranid_free_miner_l_solid_deepspace_largegroup" name="{20204,5601}" comment="very few">
//     <orders>
//       <order order="MiningRoutine" default="true">
//         <param name="minbuy" value="0"/>
//         <param name="maxbuy" value="5"/>
//         <param name="minsell" value="0"/>
//         <param name="maxsell" value="5"/>
//       </order>
//     </orders>
//     <category faction="paranid" tags="[freighter, miner, deepspace]" size="ship_l"/>
//     <quota galaxy="1"/>
//     <time interval="3600"/>
//     <location class="galaxy" macro="xu_ep2_universe_macro" faction="paranid" relation="member" comparison="ge" regionbasket="minerals"/>
//     <environment buildatshipyard="true"/>
//     <basket basket="minerals"/>
//     <encounters id="mining_group_large"/>
//     <ship>
//       <select faction="paranid" tags="[miner, solid]" size="ship_l"/>
//       <loadout>
//         <level min="0" max="0.7"/>
//       </loadout>
//       <owner exact="paranid" overridenpc="true"/>
//     </ship>
//     <subordinates>
//       <subordinate job="paranid_miningfleet_ml_solid_large"/>
//       <subordinate job="paranid_fighter_escort_s_miningfleet"/>
//     </subordinates>
//   </job>
