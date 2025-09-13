#!/usr/bin/env python3
"""
Script para corrigir queries SQLx no arquivo database.rs
"""

import re

def fix_database_queries():
    file_path = "src/database.rs"
    
    with open(file_path, 'r', encoding='utf-8') as f:
        content = f.read()
    
    # Padrões para substituir
    patterns = [
        # sqlx::query! -> sqlx::query
        (r'sqlx::query!\(\s*', 'sqlx::query('),
        
        # sqlx::query_as! -> sqlx::query_as::<_, Type>
        (r'sqlx::query_as!\(\s*(\w+),', r'sqlx::query_as::<_, \1>('),
        
        # Remover parâmetros inline e usar .bind()
        (r'(\w+),\s*(\w+)\s*\)', r'\1\n    .bind(\2)'),
        
        # Corrigir fetch_one com get()
        (r'Ok\(result\.id\)', 'Ok(result.get("id"))'),
        
        # Corrigir fetch_all
        (r'\.fetch_all\(pool\)', '.fetch_all(pool)'),
        
        # Corrigir fetch_optional
        (r'\.fetch_optional\(pool\)', '.fetch_optional(pool)'),
    ]
    
    for pattern, replacement in patterns:
        content = re.sub(pattern, replacement, content, flags=re.MULTILINE)
    
    with open(file_path, 'w', encoding='utf-8') as f:
        f.write(content)
    
    print("Queries corrigidas!")

if __name__ == "__main__":
    fix_database_queries()
