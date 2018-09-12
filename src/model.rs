#[derive(Serialize, Deserialize)]
pub struct ClassifyResult {
    pub name: String,
    pub prob: f32,
}

#[derive(Serialize, Deserialize)]
pub struct ClassifyResults {
    pub results: Vec<ClassifyResult>
}

impl ClassifyResults {
    pub fn dummy() -> ClassifyResults {
        // let mut results_vec = Vec::new();
        let results_vec = vec![
            ClassifyResult{ name: "アダルト".to_string(), prob: 0.2},
            ClassifyResult{ name: "災害 事故 事件".to_string(), prob: 0.02},
            ClassifyResult{ name: "訃報 葬儀 闘病".to_string(), prob: 0.02},
            ClassifyResult{ name: "虐待 いじめ 暴力".to_string(), prob: 0.9},

            ClassifyResult{ name: "オカルト ゴシップ".to_string(), prob: 0.2},
            ClassifyResult{ name: "宗教".to_string(), prob: 0.2},
            ClassifyResult{ name: "人種差別 部落問題".to_string(), prob: 0.02},
            ClassifyResult{ name: "国際批判 世界情勢".to_string(), prob: 0.02},

            ClassifyResult{ name: "政治".to_string(), prob: 0.9},
            ClassifyResult{ name: "薬物 ドラッグ".to_string(), prob: 0.02},
            ClassifyResult{ name: "違法DL".to_string(), prob: 0.02},
            ClassifyResult{ name: "出会い系".to_string(), prob: 0.02},

            ClassifyResult{ name: "法律相談".to_string(), prob: 0.02},
            ClassifyResult{ name: "タバコ".to_string(), prob: 0.02},
            ClassifyResult{ name: "アルコール".to_string(), prob: 0.02},
            ClassifyResult{ name: "ギャンブル".to_string(), prob: 0.02},
        ];
        let classify_results = ClassifyResults{results: results_vec};

        classify_results
    }
}
