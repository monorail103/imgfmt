/**
 * ファイル名として有効かどうかをチェックする関数
 * @ param filename: チェックするファイル名
 * @return: 有効なファイル名ならtrue、無効ならfalse
 */
pub fn is_valid_filename(filename: &str) -> bool {
    // 空でないこと、長すぎないこと（例: 255文字以下）
    if filename.is_empty() || filename.len() > 255 {
        return false;
    }
    // パス・トラバーサル攻撃を防ぐ
    if filename.contains("..") || filename.contains('/') || filename.contains('\\') {
        return false;
    }
    // 制御文字などが含まれていないことを確認
    filename.chars().all(|c| !c.is_control())
}